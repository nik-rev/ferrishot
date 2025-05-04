//! Main logic for the application, handling of events and mutation of the state

use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::Cli;
use crate::Config;
use crate::config::KeyAction;
use crate::config::Place;
use crate::ui;
use crate::ui::popup;
use iced::Length::Fill;
use iced::Renderer;
use iced::Subscription;
use iced::Theme;
use iced::mouse::Interaction;
use iced::widget::Canvas;
use iced::window;
use iced::{
    Rectangle,
    widget::{Action, canvas},
};
use ui::popup::PickCorner;

use crate::message::Message;
use crate::screenshot::Screenshot;
use iced::widget::Stack;
use iced::{Point, Size, Task};

use crate::rect::Direction;
use crate::rect::RectangleExt;
use crate::ui::selection::Selection;

use super::Errors;
use super::popup::Popup;
use super::selection::OptionalSelectionExt as _;
use super::selection::SelectionKeysState;

/// The image to save to a file, chosen by the user in a file picker.
///
/// Unfortunately, there is simply no way to communicate something from
/// the inside of an iced application to the outside: i.e. "Return" something
/// from an iced program exiting. So we have to use a global variable for this.
///
/// This global is mutated just *once* at the end of the application's lifetime,
/// when the window closes.
///
/// It is then accessed just *once* to open the file dialog and let the user pick
/// where they want to save their image.
///
/// Yes, at the moment we want this when using Ctrl + S to save as file:
/// 1. Close the application to save the file and generate the image we'll save
/// 2. Open the file explorer, and save the image to the specified path
///
/// When the file explorer is spawned from the inside of an iced window, closing
/// this window will then also close the file explorer. It means that we can't
/// close the window and then spawn an explorer.
///
/// The other option is to have both windows open at the same time. But this
/// would be really odd. First of all, we will need to un-fullscreen the App
/// because the file explorer can spawn under the app.
///
/// This is going to be sub-optimal. Currently, we give off the illusion of
/// drawing shapes and rectangles on top of the desktop. It is not immediately
/// obvious that the app is just another window which is full-screen.
/// Doing the above would break that illusion.
///
/// Ideally, we would be able to spawn a file explorer *above* the window without
/// having to close this. But this seems to not be possible. Perhaps in the
/// future there will be some kind of file explorer Iced widget that we
/// can use instead of the native file explorer.
pub static SAVED_IMAGE: std::sync::OnceLock<image::DynamicImage> = std::sync::OnceLock::new();

/// Holds the state for ferrishot
#[derive(Debug)]
pub struct App {
    /// If an image is in the process of being uploaded (but hasn't yet)
    pub is_uploading_image: bool,
    /// When the application was launched
    pub time_started: Instant,
    /// How long has passed since starting ferrishot
    pub time_elapsed: Duration,
    /// Config of the app
    pub config: Arc<Config>,
    /// A list of messages which obtained while the debug overlay is active
    pub logged_messages: Vec<Message>,
    /// How many selections were created throughout the
    /// lifetime of the App
    pub selections_created: usize,
    /// The full screenshot of the monitor from which ferrishot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    pub image: Screenshot,
    /// Area of the screen that is selected for capture
    pub selection: Option<Selection>,
    /// Errors to display to the user
    pub errors: Errors,
    /// Whether to show an overlay with additional information (F12)
    pub show_debug_overlay: bool,
    /// Command line arguments passed
    pub cli: Arc<Cli>,

    /// Currently opened popup
    pub popup: Option<Popup>,
}

impl App {
    /// Create a new `App`
    #[must_use]
    pub fn new(cli: Arc<Cli>, config: Arc<Config>) -> Self {
        Self {
            is_uploading_image: false,
            time_started: Instant::now(),
            time_elapsed: Duration::default(),
            selection: cli.region.map(|rect| Selection {
                theme: config.theme,
                is_first: true,
                accept_on_select: cli.accept_on_select,
                rect,
                status: ui::selection::SelectionStatus::default(),
            }),
            logged_messages: vec![],
            selections_created: 0,
            // FIXME: Currently the app cannot handle when the resolution is very small
            // if a path was passed and the path contains a valid image
            image: cli
                .file
                .as_ref()
                .and_then(|path| image::ImageReader::open(path).ok())
                .and_then(|reader| reader.decode().ok())
                .map(|img| Screenshot::new(img.width(), img.height(), img.into_rgba8().into_raw()))
                // Default is taking a screenshot of the desktop
                .unwrap_or_default(),
            errors: Errors::default(),
            show_debug_overlay: cli.debug,
            config,
            cli,
            popup: None,
        }
    }

    /// Close the app
    ///
    /// This is like `iced::exit`, but it does not cause a segfault in special
    /// circumstances <https://github.com/iced-rs/iced/issues/2625>
    fn exit() -> Task<Message> {
        iced::window::get_latest().then(|id| iced::window::close(id.expect("window to exist")))
    }

    /// This method is used to keep track of time / how much time has passed since start
    /// of the program, using this for animations.
    pub fn subscription(&self) -> Subscription<crate::Message> {
        window::frames().map(crate::Message::Tick)
    }

    /// Renders the app
    pub fn view(&self) -> iced::Element<Message> {
        Stack::new()
            // taken screenshot in the background
            .push(super::BackgroundImage {
                image_handle: Screenshot::clone(&self.image).into(),
            })
            // Shade in the background + global event handler + selection renderer
            .push(Canvas::new(self).width(Fill).height(Fill))
            // information popup with basic tips
            .push_maybe(
                (self.popup.is_none() && self.selection.is_none())
                    .then(|| super::welcome_message(self)),
            )
            // errors
            .push(self.errors.view(self))
            // icons around the selection
            .push_maybe(
                self.selection
                    .filter(|sel| sel.is_idle() && self.config.selection_icons)
                    .map(|sel| {
                        super::SelectionIcons {
                            app: self,
                            image_width: self.image.width() as f32,
                            image_height: self.image.height() as f32,
                            selection_rect: sel.rect.norm(),
                        }
                        .view()
                    }),
            )
            // size indicator
            .push_maybe(
                self.selection
                    .filter(|_| self.config.size_indicator)
                    .get()
                    .map(|(sel, sel_is_some)| {
                        super::size_indicator(self, sel.rect.norm(), sel_is_some)
                    }),
            )
            .push_maybe(self.popup.as_ref().map(|popup| {
                match popup {
                    Popup::Letters(state) => ui::popup::Letters {
                        app: self,
                        pick_corner: state.picking_corner,
                    }
                    .view(),
                    Popup::ImageUploaded(state) => ui::popup::ImageUploaded {
                        app: self,
                        qr_code_data: &state.url.0,
                        data: &state.url.1,
                        url_copied: state.has_copied_link,
                    }
                    .view(),
                    Popup::KeyCheatsheet => ui::popup::KeybindingsCheatsheet {
                        theme: &self.config.theme,
                    }
                    .view(),
                }
            }))
            // debug overlay
            .push_maybe(self.show_debug_overlay.then(|| super::debug_overlay(self)))
            .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use crate::message::Handler as _;

        match message {
            Message::ClosePopup => {
                self.popup = None;
            }
            Message::Tick(instant) => {
                self.time_elapsed = instant.duration_since(self.time_started);
            }
            Message::KeyCheatsheet(key_cheatsheet) => {
                return key_cheatsheet.handle(self);
            }
            Message::Selection(selection) => {
                return selection.handle(self);
            }
            Message::SizeIndicator(size_indicator) => {
                return size_indicator.handle(self);
            }
            Message::ImageUploaded(image_uploaded) => {
                return image_uploaded.handle(self);
            }
            Message::Letters(letters) => {
                return letters.handle(self);
            }
            Message::NoOp => (),
            Message::KeyBind { action, count } => match action {
                KeyAction::NoOp => {}
                KeyAction::OpenKeybindingsCheatsheet => {
                    self.popup = Some(Popup::KeyCheatsheet);
                }
                KeyAction::CopyToClipboard => {
                    let Some(selection) = self.selection.map(Selection::norm) else {
                        self.errors.push("There is no selection to copy");
                        return Task::none();
                    };

                    let cropped_image = selection.process_image(
                        self.image.width(),
                        self.image.height(),
                        self.image.bytes(),
                    );

                    let image_data = arboard::ImageData {
                        width: cropped_image.width() as usize,
                        height: cropped_image.height() as usize,
                        bytes: std::borrow::Cow::Borrowed(cropped_image.as_bytes()),
                    };

                    // #[cfg_attr(
                    //     target_os = "macos",
                    //     expect(unused_variables, reason = "it is used on other platforms")
                    // )]
                    match crate::clipboard::set_image(image_data) {
                        Ok(_img_path) => {
                            // send desktop notification if possible, this is
                            // just a decoration though so it's ok if we fail to do this
                            // let mut notify = notify_rust::Notification::new();

                            // notify.summary(&format!(
                            //     "Copied image to clipboard {w}px * {h}px",
                            //     w = cropped_image.width(),
                            //     h = cropped_image.height()
                            // ));

                            // images are not supported on macos
                            // #[cfg(not(target_os = "macos"))]
                            // notify.image_path(&img_path.to_string_lossy());

                            // let _ = notify.show();

                            return Self::exit();
                        }
                        Err(err) => {
                            self.errors.push(format!("Could not copy the image: {err}"));
                        }
                    }
                }
                KeyAction::ToggleDebugOverlay => {
                    self.show_debug_overlay = !self.show_debug_overlay;
                }
                KeyAction::ClearSelection => {
                    self.selection = None;
                }
                KeyAction::SelectFullScreen => {
                    self.selection = Some(
                        Selection::new(
                            Point { x: 0.0, y: 0.0 },
                            &self.config.theme,
                            self.selections_created == 0,
                            self.cli.accept_on_select,
                        )
                        .with_size(|_| Size {
                            width: self.image.width() as f32,
                            height: self.image.height() as f32,
                        }),
                    );
                    self.selections_created += 1;
                }
                KeyAction::SaveScreenshot => {
                    let Some(selection) = self.selection.as_ref().map(|sel| Selection::norm(*sel))
                    else {
                        self.errors
                            .push("Selection does not exist. There is nothing to copy!");
                        return Task::none();
                    };

                    let cropped_image = selection.process_image(
                        self.image.width(),
                        self.image.height(),
                        self.image.bytes(),
                    );

                    let _ = SAVED_IMAGE.set(cropped_image);

                    return Self::exit();
                }
                KeyAction::Exit => return Self::exit(),
                KeyAction::SetWidth => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let image_width = self.image.width() as f32;
                    let sel = selection.norm();

                    *selection = sel.with_width(|_| (count as f32).min(image_width - sel.rect.x));
                }
                KeyAction::SetHeight => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let image_height = self.image.height() as f32;
                    let sel = selection.norm();

                    *selection = sel.with_height(|_| (count as f32).min(image_height - sel.rect.y));
                }
                KeyAction::Goto(place) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let image_height = self.image.height() as f32;
                    let image_width = self.image.width() as f32;

                    match place {
                        Place::Center => {
                            selection.rect.x = (image_width - selection.rect.width) / 2.0;
                            selection.rect.y = (image_height - selection.rect.height) / 2.0;
                        }
                        Place::XCenter => {
                            selection.rect.x = (image_width - selection.rect.width) / 2.0;
                        }
                        Place::YCenter => {
                            selection.rect.y = (image_height - selection.rect.height) / 2.0;
                        }
                        Place::TopLeft => {
                            selection.rect.x = 0.0;
                            selection.rect.y = 0.0;
                        }
                        Place::TopRight => {
                            selection.rect.x = image_width - selection.rect.width;
                            selection.rect.y = 0.0;
                        }
                        Place::BottomLeft => {
                            selection.rect.x = 0.0;
                            selection.rect.y = image_height - selection.rect.height;
                        }
                        Place::BottomRight => {
                            selection.rect.x = image_width - selection.rect.width;
                            selection.rect.y = image_height - selection.rect.height;
                        }
                        Place::Top => {
                            selection.rect.y = 0.0;
                        }
                        Place::Bottom => {
                            selection.rect.y = image_height - selection.rect.height;
                        }
                        Place::Left => {
                            selection.rect.x = 0.0;
                        }
                        Place::Right => {
                            selection.rect.x = image_width - selection.rect.width;
                        }
                    }
                }
                KeyAction::Move(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let image_width = self.image.width() as f32;
                    let image_height = self.image.height() as f32;
                    let amount = amount as f32 * count as f32;
                    let sel = selection.norm();

                    *selection = match direction {
                        Direction::Up => sel.with_y(|y| (y - amount).max(0.0)),
                        Direction::Down => {
                            sel.with_y(|y| (y + amount).min(image_height - sel.rect.height))
                        }
                        Direction::Left => sel.with_x(|x| (x - amount).max(0.0)),
                        Direction::Right => {
                            sel.with_x(|x| (x + amount).min(image_width - sel.rect.width))
                        }
                    }
                }
                KeyAction::Extend(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let image_height = self.image.height() as f32;
                    let image_width = self.image.width() as f32;
                    let sel = selection.norm();
                    let amount = amount as f32 * count as f32;

                    *selection = match direction {
                        Direction::Up => sel
                            .with_y(|y| (y - amount).max(0.0))
                            .with_height(|h| (h + amount).min(sel.rect.y + sel.rect.height)),
                        Direction::Down => {
                            sel.with_height(|h| (h + amount).min(image_height - sel.rect.y))
                        }
                        Direction::Left => sel
                            .with_x(|x| (x - amount).max(0.0))
                            .with_width(|w| (w + amount).min(sel.rect.x + sel.rect.width)),
                        Direction::Right => {
                            sel.with_width(|w| (w + amount).min(image_width - sel.rect.x))
                        }
                    }
                }
                KeyAction::Shrink(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.errors.push("Nothing is selected.");
                        return Task::none();
                    };
                    let sel = selection.norm();
                    let amount = amount as f32 * count as f32;

                    *selection = match direction {
                        Direction::Up => sel
                            .with_y(|y| (y + amount).min(sel.rect.y + sel.rect.height))
                            .with_height(|h| (h - amount).max(0.0)),
                        Direction::Down => sel.with_height(|h| (h - amount).max(0.0)),
                        Direction::Left => sel
                            .with_x(|x| (x + amount).min(sel.rect.x + sel.rect.width))
                            .with_width(|w| (w - amount).max(0.0)),
                        Direction::Right => sel.with_width(|w| (w - amount).max(0.0)),
                    }
                }
                KeyAction::PickTopLeftCorner => {
                    self.popup = Some(Popup::Letters(popup::letters::State {
                        picking_corner: PickCorner::TopLeft,
                    }));
                }
                KeyAction::PickBottomRightCorner => {
                    self.popup = Some(Popup::Letters(popup::letters::State {
                        picking_corner: PickCorner::BottomRight,
                    }));
                }
                KeyAction::UploadScreenshot => {
                    let Some(selection) = self.selection.as_ref().map(|sel| Selection::norm(*sel))
                    else {
                        self.errors
                            .push("Select something on the screen to upload it");
                        return Task::none();
                    };

                    self.is_uploading_image = true;

                    let cropped_image = selection.process_image(
                        self.image.width(),
                        self.image.height(),
                        self.image.bytes(),
                    );

                    let tempfile = match tempfile::TempDir::new() {
                        Ok(tempdir) => tempdir.into_path().join("ferrishot-screenshot.png"),
                        Err(err) => {
                            self.errors.push(err.to_string());
                            self.is_uploading_image = false;
                            return Task::none();
                        }
                    };

                    // TODO: add config option for setting the format to upload with
                    if let Err(err) =
                        cropped_image.save_with_format(&tempfile, image::ImageFormat::Png)
                    {
                        self.errors.push(err.to_string());
                    }

                    return Task::future(async move {
                        {
                            let file = tempfile;
                            let file_size = file.metadata().map(|meta| meta.len()).unwrap_or(0);

                            match crate::image_upload::upload(&file).await {
                                Ok(image_uploaded) => Message::ImageUploaded(
                                    popup::image_uploaded::Message::ImageUploaded(
                                        popup::image_uploaded::ImageUploadedData {
                                            image_uploaded,
                                            uploaded_image: iced::widget::image::Handle::from_path(
                                                &file,
                                            ),
                                            height: cropped_image.height(),
                                            width: cropped_image.width(),
                                            file_size,
                                        },
                                    ),
                                ),
                                Err(err) => {
                                    Message::Error(err.into_iter().next().unwrap_or_default())
                                }
                            }
                        }
                    });
                }
            },
            Message::Error(err) => {
                self.errors.push(err);
            }
        }

        Task::none()
    }
}

/// Holds information about the mouse
#[derive(Default, Debug, Clone)]
pub struct AppKeysState {
    /// Left mouse click is currently being held down
    pub is_left_down: bool,
    /// How many times to execute the next motion
    pub motion_count: Option<u32>,
    /// The last key that was pressed
    pub last_key_pressed: Option<iced::keyboard::Key>,
}

impl canvas::Program<Message> for App {
    type State = (AppKeysState, SelectionKeysState);

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        if let Some(sel) = self.selection.map(Selection::norm) {
            sel.draw(&mut frame, bounds);
        } else {
            // usually the selection is responsible for drawing shade around itself
            // However here we don't have selection, so just draw the shade on the entire screen
            frame.fill_rectangle(
                bounds.position(),
                bounds.size(),
                self.config.theme.non_selected_region,
            );
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<Action<Message>> {
        use iced::Event::{Keyboard, Mouse, Touch};
        use iced::keyboard::Event::KeyPressed;
        use iced::keyboard::Key::Named;
        use iced::keyboard::Modifiers;
        use iced::keyboard::key::Named::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Shift};
        use iced::mouse::Button::Left;
        use iced::mouse::Event::ButtonPressed;
        use iced::mouse::Event::ButtonReleased;
        use iced::touch::Event::{FingerLifted, FingerPressed};

        // Handle popups. Esc = close popup
        //
        // Events will still be forwarded to the canvas even if we have a popup
        if self.popup.is_some() {
            if let Keyboard(KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
                ..
            }) = event
            {
                return Some(Action::publish(Message::ClosePopup));
            }

            return None;
        }

        let (state, selection_state) = state;

        if let Some(sel) = self.selection {
            if let Some(action) = sel.update(selection_state, event, bounds, cursor) {
                return Some(action);
            }
        }

        // handle the number pressed
        //
        // pressing numbers will have an effect, e.g. `200j` will
        // move the selection down by 200px
        if let Keyboard(KeyPressed {
            key: iced::keyboard::Key::Character(ch),
            ..
        }) = event
        {
            if let Ok(number_pressed) = ch.parse::<u32>() {
                if let Some(motion_count) = state.motion_count.as_mut() {
                    *motion_count = *motion_count * 10 + number_pressed;
                } else {
                    state.motion_count = Some(number_pressed);
                }
            }
        }

        // handle keybindings
        if let Keyboard(KeyPressed {
            modifiers,
            modified_key,
            key,
            ..
        }) = event
        {
            let mut modifiers = *modifiers;

            // Shift key does not matter. For example:
            // - pressing `<` and the `SHIFT` modifier will be pressed
            // - `G` will also trigger the `SHIFT` modifier
            //
            // However, we are going to hard-code the shift modifier to not be removed for the
            // arrow keys
            if !matches!(key, Named(ArrowLeft | ArrowDown | ArrowRight | ArrowUp)) {
                modifiers.remove(Modifiers::SHIFT);
            }

            if let Some(action) = state
                .last_key_pressed
                .as_ref()
                .and_then(|last_key_pressed| {
                    self.config.keys.get(
                        last_key_pressed.clone(),
                        Some(modified_key.clone()),
                        modifiers,
                    )
                })
                .or_else(|| self.config.keys.get(modified_key.clone(), None, modifiers))
            {
                // the last key pressed needs to be reset for it to be
                // correct in future invocations
                //
                // For example if I press `gg`, and it activates some keybinding
                // I would have to press `gg` *again* to active it.
                //
                // If we did not reset, then `ggg` would trigger the `gg` keybindings
                // twice
                state.last_key_pressed = None;

                let count = state.motion_count.unwrap_or(1);
                state.motion_count = None;

                return Some(Action::publish(Message::KeyBind {
                    action: action.clone(),
                    count,
                }));
            }

            // the "Shift" is already included in the modifiers
            //
            // Otherwise, when pressing 'G' for instance it would first set
            // - `last_key_pressed = Shift` then
            // - `last_key_pressed = 'G'`
            if *modified_key != Named(Shift) {
                state.last_key_pressed = Some(modified_key.clone());
            }
        }

        // Create the selection when it does not exist yet

        let message = match event {
            Touch(FingerPressed { .. }) | Mouse(ButtonPressed(Left)) => {
                state.is_left_down = true;
                Message::Selection(Box::new(ui::selection::Message::CreateSelection(
                    cursor.position()?,
                )))
            }
            Touch(FingerLifted { .. }) | Mouse(ButtonReleased(Left)) => {
                state.is_left_down = false;
                Message::NoOp
            }
            _ => return None,
        };

        Some(Action::publish(message))
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Interaction {
        if let Some(Popup::ImageUploaded(_)) = self.popup {
            Interaction::default()
        } else {
            self.selection
                .map(Selection::norm)
                .map_or(Interaction::Crosshair, |sel| sel.mouse_interaction(cursor))
        }
    }
}
