//! Main logic for the application, handling of events and mutation of the state

use crate::CONFIG;
use crate::config::KeyAction;
use crate::config::Place;
use crate::widget::PickCorner;
use crate::widget::selection::Speed;
use iced::Length;
use iced::Renderer;
use iced::Theme;
use iced::mouse::Interaction;
use iced::widget::Canvas;
use iced::{
    Rectangle,
    widget::{Action, canvas},
};

use crate::message::Message;
use crate::screenshot::Screenshot;
// use crate::widget::selection::selection_lock::OptionalSelectionExt;
use iced::widget::Stack;
use iced::{Point, Size, Task};

use crate::rect::RectangleExt;
use crate::rect::{Direction, Side, SideOrCorner};
use crate::widget::selection::{Selection, SelectionStatus};

use super::Errors;
use super::selection::OptionalSelectionExt as _;

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
#[derive(Debug, Default)]
pub struct App {
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
    /// Shows a grid of letters on the screen, pressing 3 letters in a row
    /// allows accessing 25 * 25 * 25 = 15,625 different locations
    pub picking_corner: Option<PickCorner>,
    /// A link to the uploaded image
    pub uploaded_url: Option<String>,
}

impl App {
    /// Create an empty selection at the current position
    pub fn create_selection_at(&mut self, create_selection_at: Point) {
        let mut selection = Selection::new(create_selection_at);
        selection.status = SelectionStatus::Create;
        self.selections_created += 1;
        self.selection = Some(selection);
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, other: Point) {
        self.selection = self.selection.take().map(|selected_region| {
            let width = other.x - selected_region.rect.x;
            let height = other.y - selected_region.rect.y;

            selected_region.with_size(|_| Size { width, height })
        });
    }

    /// Close the app
    ///
    /// This is like `iced::exit`, but it does not cause a segfault in special
    /// circumstances <https://github.com/iced-rs/iced/issues/2625>
    fn exit() -> Task<Message> {
        iced::window::get_latest().then(|id| iced::window::close(id.expect("window to exist")))
    }

    /// Renders the app
    pub fn view(&self) -> iced::Element<Message> {
        Stack::new()
            // taken screenshot in the background
            .push(super::BackgroundImage {
                image_handle: Screenshot::clone(&self.image).into(),
            })
            // event handler + shade in the background if no selection
            .push(Canvas::new(self).width(Length::Fill).height(Length::Fill))
            // border around the selection
            .push_maybe(self.selection.as_ref().map(|sel| sel.view()))
            // information popup, when there is no selection
            .push_maybe(self.selection.is_none().then(|| {
                super::WelcomeMessage {
                    image_width: self.image.width(),
                    image_height: self.image.height(),
                }
                .view()
            }))
            // errors
            .push(self.errors.view(self.image.width()))
            // icons around the selection
            .push_maybe(self.selection.filter(|sel| sel.is_idle()).map(|sel| {
                super::SelectionIcons {
                    image_width: self.image.width() as f32,
                    image_height: self.image.height() as f32,
                    selection_rect: sel.rect.norm(),
                }
                .view()
            }))
            // grid of letters to precisely choose a location
            .push_maybe(
                self.picking_corner
                    .map(|pick_corner| crate::widget::Letters { pick_corner }.view()),
            )
            // size indicator
            .push_maybe(self.selection.filter(|_| CONFIG.size_indicator).get().map(
                |(sel, sel_is_some)| {
                    super::SizeIndicator {
                        image_height: self.image.height(),
                        image_width: self.image.width(),
                        selection_rect: sel.rect.norm(),
                        sel_is_some,
                    }
                    .view()
                },
            ))
            .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LettersAbort => {
                self.picking_corner = None;
            }
            Message::LettersPick { point } => {
                let sel = self.selection.map(Selection::norm).unwrap_or_default();
                let x = point.x;
                let y = point.y;
                if let Some(pick_corner) = self.picking_corner {
                    match pick_corner {
                        PickCorner::TopLeft => {
                            self.selection = Some(sel.with_x(|_| x).with_y(|_| y));
                        }
                        PickCorner::BottomRight => {
                            self.selection = Some(
                                sel.with_height(|_| y - sel.rect.y)
                                    .with_width(|_| x - sel.rect.x),
                            );
                        }
                    }
                };
                self.picking_corner = None;
            }
            Message::ResizeVertically {
                new_height,
                sel_is_some,
            } => {
                let sel = self.selection.unlock(sel_is_some);

                // what is the minimum value for `new_height` that would make
                // this overflow vertically?
                // We want to make sure the selection cannot get bigger than that.
                let new_height =
                    new_height.min((sel.norm().rect.y + sel.norm().rect.height) as u32);

                let dy = new_height as f32 - sel.norm().rect.height;
                *sel = sel
                    .norm()
                    .with_height(|_| new_height as f32)
                    .with_y(|y| y - dy);
            }
            Message::ResizeHorizontally {
                new_width,
                sel_is_some,
            } => {
                let sel = self.selection.unlock(sel_is_some);

                // what is the minimum value for `new_width` that would make
                // this overflow vertically?
                // We want to make sure the selection cannot get bigger than that.
                let new_width = new_width.min((sel.norm().rect.x + sel.norm().rect.width) as u32);

                let dx = new_width as f32 - sel.norm().rect.width;
                *sel = sel
                    .norm()
                    .with_width(|_| new_width as f32)
                    .with_x(|x| x - dx);
            }
            Message::NoOp => (),
            Message::LeftMouseDown(cursor) => {
                if let Some((cursor, side, rect)) = cursor.position().and_then(|cursor_pos| {
                    self.selection.as_mut().and_then(|selected_region| {
                        selected_region
                            .corners()
                            .side_at(cursor_pos)
                            .map(|l| (cursor_pos, l, selected_region))
                    })
                }) {
                    // Resize selection

                    rect.status = SelectionStatus::Resize {
                        initial_rect: rect.norm().rect,
                        initial_cursor_pos: cursor,
                        resize_side: side,
                    };
                } else if let Some((cursor, selected_region)) = self
                    .selection
                    .as_mut()
                    .and_then(|sel| sel.cursor_in_selection_mut(cursor))
                {
                    // Move selection

                    selected_region.status = SelectionStatus::Move {
                        initial_rect_pos: selected_region.norm().pos(),
                        initial_cursor_pos: cursor,
                    };
                } else if let Some(cursor_position) = cursor.position() {
                    // Create selection

                    self.selection =
                        Some(Selection::new(cursor_position).with_status(SelectionStatus::Create));

                    self.selections_created += 1;
                }
            }
            Message::EnterIdle => {
                if let Some(selection) = self.selection.as_mut() {
                    selection.status = SelectionStatus::Idle;
                }
            }
            Message::MoveSelection {
                current_cursor_pos,
                initial_cursor_pos,
                current_selection,
                initial_rect_pos,
                speed,
            } => {
                let mut new_selection = current_selection.with_pos(|_| {
                    initial_rect_pos + ((current_cursor_pos - initial_cursor_pos) * speed.speed())
                });

                let old_x = new_selection.rect.x as u32;
                let old_y = new_selection.rect.y as u32;

                // if any of these actually get changed we are going to set the new selection status.

                new_selection.rect.x = new_selection
                    .rect
                    .x
                    .min(self.image.width() as f32 - new_selection.rect.width)
                    .max(0.0);

                new_selection.rect.y = new_selection
                    .rect
                    .y
                    .min(self.image.height() as f32 - new_selection.rect.height)
                    .max(0.0);

                if new_selection.rect.y as u32 != old_y || new_selection.rect.x as u32 != old_x {
                    new_selection.status = SelectionStatus::Move {
                        initial_rect_pos: new_selection.pos(),
                        initial_cursor_pos: current_cursor_pos,
                    }
                }

                if speed
                    == (Speed::Slow {
                        has_speed_changed: true,
                    })
                {
                    new_selection.status = SelectionStatus::Move {
                        initial_rect_pos: current_selection.pos(),
                        initial_cursor_pos: current_cursor_pos,
                    }
                }

                self.selection = Some(new_selection);
            }
            Message::KeyBind { action, count } => match action {
                KeyAction::ClearSelection => {
                    self.selection = None;
                }
                KeyAction::SelectFullScreen => {
                    self.selection = Some(Selection::new(Point { x: 0.0, y: 0.0 }).with_size(
                        |_| Size {
                            width: self.image.width() as f32,
                            height: self.image.height() as f32,
                        },
                    ));
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

                    #[cfg_attr(
                        target_os = "macos",
                        expect(unused_variables, reason = "it is used on other platforms")
                    )]
                    match crate::clipboard::set_image(image_data) {
                        Ok(img_path) => {
                            // send desktop notification if possible, this is
                            // just a decoration though so it's ok if we fail to do this
                            let mut notify = notify_rust::Notification::new();

                            notify.summary(&format!(
                                "Copied image to clipboard {w}px * {h}px",
                                w = cropped_image.width(),
                                h = cropped_image.height()
                            ));

                            // images are not supported on macos
                            #[cfg(not(target_os = "macos"))]
                            notify.image_path(&img_path.to_string_lossy());

                            let _ = notify.show();

                            return Self::exit();
                        }
                        Err(err) => {
                            self.errors.push(format!("Could not copy the image: {err}"));
                        }
                    }
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
                    let sel = selection.norm();

                    *selection = match place {
                        Place::Center => sel
                            .with_x(|_| (image_width - sel.rect.width) / 2.0)
                            .with_y(|_| (image_height - sel.rect.height) / 2.0),
                        Place::XCenter => sel.with_x(|_| (image_width - sel.rect.width) / 2.0),
                        Place::YCenter => sel.with_y(|_| (image_height - sel.rect.height) / 2.0),
                        Place::TopLeft => sel.with_x(|_| 0.0).with_y(|_| 0.0),
                        Place::TopRight => {
                            sel.with_x(|_| image_width - sel.rect.width).with_y(|_| 0.0)
                        }
                        Place::BottomLeft => sel
                            .with_x(|_| 0.0)
                            .with_y(|_| image_height - sel.rect.height),
                        Place::BottomRight => sel
                            .with_x(|_| image_width - sel.rect.width)
                            .with_y(|_| image_height - sel.rect.height),
                        Place::Top => sel.with_y(|_| 0.0),
                        Place::Bottom => sel.with_y(|_| image_height - sel.rect.height),
                        Place::Left => sel.with_x(|_| 0.0),
                        Place::Right => sel.with_x(|_| image_width - sel.rect.width),
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
                    self.picking_corner = Some(PickCorner::TopLeft);
                }
                KeyAction::PickBottomRightCorner => {
                    self.picking_corner = Some(PickCorner::BottomRight);
                }
            },
            Message::ExtendNewSelection(new_mouse_position) => {
                self.update_selection(new_mouse_position);
            }
            Message::Upload => {
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

                let tempfile = match tempfile::TempDir::new() {
                    Ok(tempdir) => tempdir.into_path().join("ferrishot-screenshot.png"),
                    Err(err) => {
                        self.errors.push(err.to_string());
                        return Task::none();
                    }
                };

                if let Err(err) = cropped_image.save_with_format(&tempfile, image::ImageFormat::Png)
                {
                    self.errors.push(err.to_string());
                }

                return Task::future(async move {
                    {
                        let file = tempfile;
                        let response = CONFIG
                            .default_image_upload_provider
                            .upload_image(&file)
                            .await;

                        match response {
                            Ok(url) => Message::ImageUploaded { url },
                            Err(err) => Message::Error(err.to_string()),
                        }
                    }
                });
            }
            Message::ImageUploaded { url } => {
                self.uploaded_url = Some(url);
            }
            Message::ExitImageUploadMenu => self.uploaded_url = None,
            Message::Error(err) => {
                self.errors.push(err);
            }
            Message::Resize {
                current_cursor_pos,
                initial_cursor_pos,
                resize_side,
                initial_rect,
                sel_is_some,
                speed,
            } => {
                let selected_region = self.selection.unlock(sel_is_some);
                let resize_speed = speed.speed();

                let dy = (current_cursor_pos.y - initial_cursor_pos.y) * resize_speed;
                let dx = (current_cursor_pos.x - initial_cursor_pos.x) * resize_speed;

                selected_region.rect = match resize_side {
                    SideOrCorner::Side(side) => match side {
                        Side::Top => initial_rect.with_height(|h| h - dy).with_y(|y| y + dy),
                        Side::Right => initial_rect.with_width(|w| w + dx),
                        Side::Bottom => initial_rect.with_height(|h| h + dy),
                        Side::Left => initial_rect.with_width(|w| w - dx).with_x(|x| x + dx),
                    },
                    SideOrCorner::Corner(corner) => corner.resize_rect(initial_rect, dy, dx),
                };

                if speed
                    == (Speed::Slow {
                        has_speed_changed: true,
                    })
                {
                    selected_region.status = SelectionStatus::Resize {
                        initial_rect: selected_region.rect,
                        initial_cursor_pos: current_cursor_pos,
                        resize_side,
                    }
                }
            }
            Message::ResizeToCursor {
                cursor_pos,
                selection,
                sel_is_some,
            } => {
                let (corner_point, corners) = selection.corners().nearest_corner(cursor_pos);
                let sel = self.selection.unlock(sel_is_some);

                sel.rect = corners.resize_rect(
                    selection.rect,
                    cursor_pos.y - corner_point.y,
                    cursor_pos.x - corner_point.x,
                );

                sel.status = SelectionStatus::Resize {
                    initial_rect: sel.rect,
                    initial_cursor_pos: cursor_pos,
                    resize_side: SideOrCorner::Corner(corners),
                };
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
    type State = AppKeysState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        if self.selection.is_none() {
            // usually the selection is responsible for drawing shade around itself
            // However here we don't have selection, so just draw the shade on the entire screen
            frame.fill_rectangle(
                bounds.position(),
                bounds.size(),
                CONFIG.theme.non_selected_region,
            );
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<Action<Message>> {
        use iced::Event::{Keyboard, Mouse};
        use iced::keyboard::Event::KeyPressed;
        use iced::keyboard::Key::Named;
        use iced::keyboard::Modifiers;
        use iced::keyboard::key::Named::Shift;
        use iced::mouse::Button::Left;
        use iced::mouse::Event::ButtonPressed;
        use iced::mouse::Event::ButtonReleased;

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
            ..
        }) = event
        {
            let mut modifiers = *modifiers;

            // Shift key does not matter. For example:
            // - pressing `<` and the `SHIFT` modifier will be pressed
            // - `G` will also trigger the `SHIFT` modifier
            //
            // We also forbid the user from specifying `shift` as a modifier in their `config.kdl`
            modifiers.remove(Modifiers::SHIFT);

            if let Some(action) = state
                .last_key_pressed
                .as_ref()
                .and_then(|last_key_pressed| {
                    CONFIG.keys.get(
                        last_key_pressed.clone(),
                        Some(modified_key.clone()),
                        modifiers,
                    )
                })
                .or_else(|| CONFIG.keys.get(modified_key.clone(), None, modifiers))
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
            Mouse(ButtonPressed(Left)) => {
                state.is_left_down = true;
                Message::LeftMouseDown(cursor)
            }
            Mouse(ButtonReleased(Left)) => {
                state.is_left_down = false;
                if CONFIG.instant && self.selections_created == 1 {
                    // we have created 1 selections in total, (the current one),
                    // in which case we want to copy it to the clipboard
                    Message::KeyBind {
                        action: KeyAction::CopyToClipboard,
                        count: 1,
                    }
                } else {
                    // stop the creating of the initial selection
                    Message::EnterIdle
                }
            }
            _ => return None,
        };

        Some(Action::publish(message))
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Interaction {
        Interaction::Crosshair
    }
}
