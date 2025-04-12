//! Main logic for the application, handling of events and mutation of the state

use std::borrow::Cow;
use std::time::Instant;

use crate::SHADE_COLOR;
use crate::config::Config;
use crate::message::Message;
use crate::screenshot::RgbaHandle;
use clap::Parser as _;
use iced::keyboard::{Key, Modifiers};
use iced::mouse::{Cursor, Interaction};
use iced::widget::canvas::Path;
use iced::widget::{self, Action, canvas, stack};
use iced::{Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};

use crate::background_image::BackgroundImage;
use crate::corners::Side;
use crate::mouse::MouseState;
use crate::rectangle::RectangleExt;
use crate::selection::{Selection, SelectionStatus};

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

/// Show an error message to the user
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ErrorMessage {
    /// Error message
    message: Cow<'static, str>,
    /// When the error was created
    timestamp: Instant,
}

impl ErrorMessage {
    /// Create a new error message
    pub fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Self {
            message: message.into(),
            timestamp: Instant::now(),
        }
    }
}

/// Holds the state for ferrishot
#[derive(Debug)]
pub struct App {
    /// How many selections were created throughout the
    /// lifetime of the App
    selections_created: usize,
    /// The full screenshot of the monitor from which ferrishot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    screenshot: RgbaHandle,
    /// Area of the screen that is selected for capture
    selection: Option<Selection>,
    /// Configuration of the app
    config: Config,
    /// Errors to display to the user
    errors: Vec<ErrorMessage>,
}

impl Default for App {
    fn default() -> Self {
        let screenshot = crate::screenshot::screenshot().unwrap();
        let config = Config::parse();

        Self {
            screenshot,
            selection: None,
            config,
            selections_created: 0,
            errors: vec![],
        }
    }
}

impl App {
    /// Close the app
    ///
    /// This is like `iced::exit`, but it does not cause a segfault in special
    /// circumstances <https://github.com/iced-rs/iced/issues/2625>
    fn exit() -> Task<Message> {
        iced::window::get_latest().then(|id| iced::window::close(id.unwrap()))
    }

    /// Add a new error to the list of errors
    fn error<T: Into<Cow<'static, str>> + std::fmt::Display>(&mut self, error: T) {
        log::error!("Status Error: {error}");
        self.errors.push(ErrorMessage::new(error));
    }

    /// Retrieve all valid errors
    #[expect(dead_code)]
    fn errors(&self) -> Vec<String> {
        let now = Instant::now();
        self.errors
            .iter()
            .rev()
            .map_while(|err| {
                let time_passed = now - err.timestamp;
                (time_passed <= crate::ERROR_TIMEOUT).then_some(err.message.to_string())
            })
            .collect()
    }

    /// Renders the black tint on regions that are not selected
    fn render_shade(&self, frame: &mut canvas::Frame, bounds: Rectangle) {
        let Some(selection) = self.selection.map(Selection::normalize) else {
            frame.fill_rectangle(bounds.pos(), bounds.size(), SHADE_COLOR);
            return;
        };

        // represents the area outside of the selection
        let outside = Path::new(|p| {
            p.move_to(bounds.top_left());
            p.line_to(bounds.top_right());
            p.line_to(bounds.bottom_right());
            p.line_to(bounds.bottom_left());
            p.move_to(bounds.top_left());
            p.move_to(selection.top_left());
            p.line_to(selection.bottom_left());
            p.line_to(selection.bottom_right());
            p.line_to(selection.top_right());
            p.move_to(selection.top_left());
        });

        frame.fill(&outside, SHADE_COLOR);
    }

    /// Receives keybindings
    #[must_use]
    pub fn handle_key_press(key: Key, mods: Modifiers) -> Option<Message> {
        match (key, mods) {
            (Key::Named(iced::keyboard::key::Named::Escape), _) => Some(Message::Exit),
            (Key::Character(ch), Modifiers::CTRL) if ch == "c" => Some(Message::CopyToClipboard),
            (Key::Named(iced::keyboard::key::Named::Enter), _) => Some(Message::CopyToClipboard),
            (Key::Character(ch), Modifiers::CTRL) if ch == "s" => Some(Message::SaveScreenshot),
            _ => None,
        }
    }

    /// Renders the app
    pub fn view(&self) -> Element<Message> {
        stack![
            BackgroundImage::new(self.screenshot.clone().into()),
            canvas(self).width(Length::Fill).height(Length::Fill),
        ]
        .into()
    }

    /// Modifies the app's state
    ///
    /// # Panics
    ///
    /// Will panic if `self.selection` is `None` when sending the `Message::InitialResize`
    #[expect(clippy::needless_pass_by_value, reason = "trait function")]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => return Self::exit(),
            Message::LeftMouseDown(cursor) => {
                if let Some((cursor, side, rect)) = cursor.position().and_then(|cursor_pos| {
                    self.selection.as_mut().and_then(|selected_region| {
                        selected_region
                            .corners()
                            .side_at(cursor_pos)
                            .map(|l| (cursor_pos, l, selected_region))
                    })
                }) {
                    let resized = SelectionStatus::Resized {
                        initial_rect: rect.normalize().rect,
                        initial_cursor_pos: cursor,
                        resize_side: side,
                    };
                    rect.status = resized;
                } else if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor)
                {
                    let dragged = SelectionStatus::Dragged {
                        initial_rect_pos: selected_region.pos(),
                        initial_cursor_pos: cursor,
                    };
                    selected_region.status = dragged;
                } else if let Some(cursor_position) = cursor.position() {
                    // no region is selected, select the initial region
                    self.create_selection_at(
                        cursor_position,
                        self.selection
                            .map(|region| region.status)
                            .unwrap_or_default(),
                    );
                }
            },
            Message::LeftMouseUp => {
                if let Some(selection) = self.selection.as_mut() {
                    selection.status = SelectionStatus::Idle;
                }
            },
            Message::MovingSelection {
                current_cursor_pos,
                initial_cursor_pos,
                current_selection,
                initial_rect_pos,
            } => {
                self.selection = Some(
                    current_selection
                        .set_pos(initial_rect_pos + (current_cursor_pos - initial_cursor_pos)),
                );
            },
            Message::ExtendNewSelection(new_mouse_position) => {
                self.update_selection(new_mouse_position);
            },
            Message::CopyToClipboard => {
                let Some(selection) = self.selection.map(Selection::normalize) else {
                    self.error("There is no selection to copy");
                    return ().into();
                };

                let (width, height, pixels) = self.screenshot.raw();

                let cropped_image = selection.process_image(width, height, pixels);

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

                        notify
                            .summary(&format!("Copied image to clipboard {width}px * {height}px"));

                        // images are not supported on macos
                        #[cfg(not(target_os = "macos"))]
                        notify.image_path(&img_path.to_string_lossy());

                        let _ = notify.show();

                        return Self::exit();
                    },
                    // TODO: show error to the user in a custom widget
                    Err(err) => {
                        self.error(format!("Could not copy the image: {err}"));
                    },
                }
            },
            Message::SaveScreenshot => {
                let Some(selection) = self
                    .selection
                    .as_ref()
                    .map(|sel| Selection::normalize(*sel))
                else {
                    // TODO: instead of this, show an error to the user in
                    // a custom widget
                    return ().into();
                };

                let (width, height, pixels) = self.screenshot.raw();
                let cropped_image = selection.process_image(width, height, pixels);

                let _ = SAVED_IMAGE.set(cropped_image);

                return Self::exit();
            },
            Message::InitialResize {
                current_cursor_pos,
                initial_cursor_pos,
                resize_side,
                initial_rect,
            } => {
                let selected_region = self
                    .selection
                    .as_mut()
                    .expect("is inside `.is_some_and` guard");

                let dy = current_cursor_pos.y - initial_cursor_pos.y;
                let dx = current_cursor_pos.x - initial_cursor_pos.x;

                // To give a perspective on this math, imagine that our cursor is at the top left corner
                // and travelling diagonally down, from point (700, 700) -> (800, 800).
                //
                // In this case, the - `(current {x,y} [800 - 700] - previous {x,y} [700, 700])` will
                // both have positive `dx` and `dy` [100].
                //
                // Now imagine how the selection transforms with this, and think about it just for 1 case.
                // It will then be true for all cases

                selected_region.rect = match resize_side {
                    Side::TopLeft => initial_rect
                        .set_pos(current_cursor_pos)
                        .with_width(|w| w - dx)
                        .with_height(|h| h - dy),
                    Side::TopRight => initial_rect
                        .with_width(|w| w + dx)
                        .with_y(|y| y + dy)
                        .with_height(|h| h - dy),
                    Side::BottomLeft => initial_rect
                        .with_x(|x| x + dx)
                        .with_width(|w| w - dx)
                        .with_height(|h| h + dy),
                    Side::BottomRight => {
                        initial_rect.with_width(|w| w + dx).with_height(|h| h + dy)
                    },
                    Side::Top => initial_rect.with_height(|h| h - dy).with_y(|y| y + dy),
                    Side::Right => initial_rect.with_width(|w| w + dx),
                    Side::Bottom => initial_rect.with_height(|h| h + dy),
                    Side::Left => initial_rect.with_width(|w| w - dx).with_x(|x| x + dx),
                };
            },
            Message::Noop => (),
        }

        ().into()
    }

    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    fn cursor_in_selection(&self, cursor: Cursor) -> Option<(Point, Selection)> {
        self.selection.and_then(|sel| {
            cursor
                .position()
                .and_then(|cursor_pos| sel.contains(cursor_pos).then_some((cursor_pos, sel)))
        })
    }
    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    fn cursor_in_selection_mut(&mut self, cursor: Cursor) -> Option<(Point, &mut Selection)> {
        self.selection.as_mut().and_then(|sel| {
            cursor.position().and_then(|cursor_pos| {
                sel.normalize()
                    .contains(cursor_pos)
                    .then_some((cursor_pos, sel))
            })
        })
    }

    /// Create an empty selection at the current position
    pub fn create_selection_at(
        &mut self,
        create_selection_at: Point,
        moving_selection: SelectionStatus,
    ) {
        let mut selection = Selection::new(create_selection_at);
        selection.status = moving_selection;
        self.selections_created += 1;
        self.selection = Some(Selection::new(create_selection_at));
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, other: Point) {
        self.selection = self.selection.take().map(|selected_region| {
            #[rustfmt::skip]
            {
    // selected_region -> x1y1-------------------------x2
    //   (fixed)          |             ^
    //                    |           width            ~
    //                    |
    //                    |
    //                    | <- height                  ~
    //                    |
    //                    |                            ~
    //                    |
    //                   y2    ~      ~       ~   ~  x2y2 <- create_selection_at (can move)
            };
            let width = other.x - selected_region.x();
            let height = other.y - selected_region.y();

            Selection::default()
                .set_pos(selected_region.pos())
                .set_size(Size { width, height })
        });
    }
}

impl canvas::Program<Message> for App {
    type State = MouseState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        self.render_shade(&mut frame, bounds);

        if let Some(selection) = self.selection.map(Selection::normalize) {
            selection.render_border(&mut frame);
            selection.corners().render_circles(&mut frame);
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        self.selection
            .and_then(|sel| {
                // when we started dragging a side, even if we go outside of the bounds of that side (which
                // happens often when we are dragging the mouse fast), we don't want the cursor to change
                cursor
                    .position()
                    .and_then(|cursor| sel.corners().side_at(cursor).map(Side::mouse_icon))
                    // for example, if we start dragging top right corner, and move mouse to the
                    // top left corner, we want the cursor to switch appropriately
                    .or_else(|| {
                        if let SelectionStatus::Resized { resize_side, .. } = sel.status {
                            Some(resize_side.mouse_icon())
                        } else {
                            None
                        }
                    })
            })
            .unwrap_or_else(|| {
                let is_left_released = state.is_left_released();
                let is_moving_selection = self.selection.is_some_and(|sel| sel.status.is_dragged());

                let is_grab = (is_left_released || is_moving_selection)
                    && self.cursor_in_selection(cursor).is_some();
                if is_grab {
                    Interaction::Grab
                } else {
                    Interaction::Crosshair
                }
            })
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        use iced::Event::Mouse;

        let message = match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.left_click();
                Message::LeftMouseDown(cursor)
            },
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.left_release();
                if self.config.instant && self.selections_created == 1 {
                    Message::CopyToClipboard
                } else {
                    Message::LeftMouseUp
                }
            },
            Mouse(mouse::Event::CursorMoved { position })
                if state.is_left_clicked()
                    && self
                        .selection
                        .is_some_and(super::selection::Selection::is_resized) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Resized {
                    resize_side,
                    initial_rect,
                    initial_cursor_pos,
                } = self.selection.expect("has `.is_some()` guard").status
                else {
                    unreachable!();
                };
                Message::InitialResize {
                    current_cursor_pos: *position,
                    resize_side,
                    initial_cursor_pos,
                    initial_rect,
                }
            },
            Mouse(mouse::Event::CursorMoved { position })
                if state.is_left_clicked()
                    && self
                        .selection
                        .is_some_and(super::selection::Selection::is_dragged) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Dragged {
                    initial_rect_pos,
                    initial_cursor_pos,
                } = self.selection.expect("has `.is_some()` guard").status
                else {
                    unreachable!();
                };

                Message::MovingSelection {
                    current_cursor_pos: *position,
                    initial_cursor_pos,
                    current_selection: self.selection.expect("has `.is_some()` guard"),
                    initial_rect_pos,
                }
            },
            Mouse(mouse::Event::CursorMoved { position })
                if state.is_left_clicked()
                    && self
                        .selection
                        .is_some_and(super::selection::Selection::is_idle) =>
            {
                Message::ExtendNewSelection(*position)
            },
            _ => return None,
        };

        log::info!("Received message: {message:#?}");

        Some(Action::publish(message))
    }
}
