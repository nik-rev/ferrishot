//! Main logic for the application, handling of events and mutation of the state

use crate::message::Message;
use iced::keyboard::{Key, Modifiers};
use iced::mouse::{Cursor, Interaction};
use iced::widget::{self, Action, canvas, stack};
use iced::{Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};

use crate::background_image::BackgroundImage;
use crate::corners::Side;
use crate::mouse::MouseState;
use crate::rectangle::RectangleExt;
use crate::selection::{Selection, SelectionStatus};

/// Holds the state for ferrishot
#[derive(Debug)]
pub struct App {
    /// The full screenshot of the monitor from which ferrishot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    screenshot: widget::image::Handle,
    /// Area of the screen that is selected for capture
    selected_region: Option<Selection>,
}

impl Default for App {
    fn default() -> Self {
        let screenshot = crate::screenshot::screenshot().unwrap();
        Self {
            screenshot,
            selected_region: None,
        }
    }
}

impl App {
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
            BackgroundImage::new(self.screenshot.clone()),
            canvas(self).width(Length::Fill).height(Length::Fill),
        ]
        .into()
    }

    /// Modifies the app's state
    ///
    /// # Panics
    ///
    /// - When cannot find the cursor position
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => return iced::exit(),
            Message::LeftMouseDown(cursor) => {
                if let Some((cursor, side, rect)) = cursor.position().and_then(|cursor_pos| {
                    self.selected_region.as_mut().and_then(|selected_region| {
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
                    rect.selection_status = resized;
                } else if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor)
                {
                    let dragged = SelectionStatus::Dragged {
                        initial_rect_pos: selected_region.pos(),
                        initial_cursor_pos: cursor,
                    };
                    selected_region.selection_status = dragged;
                } else {
                    // no region is selected, select the initial region
                    let cursor_position = cursor.position().expect("cursor to be in the monitor");
                    self.create_selection_at(
                        cursor_position,
                        self.selected_region
                            .map(|region| region.selection_status)
                            .unwrap_or_default(),
                    );
                }
            },
            Message::LeftMouseUp => {
                if let Some(selection) = self.selected_region.as_mut() {
                    selection.selection_status = SelectionStatus::Idle;
                }
            },
            Message::MovingSelection {
                current_cursor_pos,
                initial_cursor_pos,
                current_selection,
                initial_rect_pos,
            } => {
                self.selected_region = Some(
                    current_selection
                        .set_pos(initial_rect_pos + (current_cursor_pos - initial_cursor_pos)),
                );
            },
            Message::ExtendNewSelection(new_mouse_position) => {
                self.update_selection(new_mouse_position);
            },
            Message::CopyToClipboard => {
                // return iced::clipboard::write("lol".to_string());
                // wl_clipboard_rs
                // let ctx = clipboard_rs::ClipboardContext::new().unwrap();
                // ctx.set_text("hello world".to_string()).unwrap();
                // TODO: send notification to the user if there is no
                // selection to copy
                // let Some(selected_region) = self.selected_region else {
                //     return ().into();
                // };
                // crate::clipboard::copy_text_to_clipboard("hello world").unwrap();
                // crate::clipboard::providers();
                // let mut clipboard = arboard::Clipboard::new().unwrap();
                // let widget::image::Handle::Rgba {
                //     width,
                //     height,
                //     ref pixels,
                //     ..
                // } = self.screenshot
                // else {
                //     unreachable!();
                // };
                // let image_data = ImageData {
                //     width: width as usize,
                //     height: height as usize,
                //     bytes: std::borrow::Cow::Borrowed(pixels),
                // };
                // clipboard.set_image(image_data).unwrap();
                crate::clipboard::set_text().unwrap();
                // // wl_clipboard_rs::copy::Options ;
                // let mut clipboard = arboard::Clipboard::new().unwrap();
                // clipboard.set_text("hello world").unwrap();
                return iced::exit();
            },
            Message::SaveScreenshot => todo!(),
            Message::InitialResize {
                current_cursor_pos,
                initial_cursor_pos,
                resize_side,
                initial_rect,
            } => {
                // FIXME: this is awkward. We know that self.selected_region EXISTS
                // when we send the Resize message, so ideally we would send a mutable
                // reference. But Messages cannot send mutable references from what I can tell
                let selected_region = self
                    .selected_region
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

                let changed_rect = match resize_side {
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
                selected_region.rect = changed_rect;
            },
        }

        ().into()
    }

    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    fn cursor_in_selection(&self, cursor: Cursor) -> Option<(Point, Selection)> {
        self.selected_region.and_then(|selected_region| {
            cursor.position().and_then(|cursor_pos| {
                selected_region
                    .contains(cursor_pos)
                    .then_some((cursor_pos, selected_region))
            })
        })
    }
    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    fn cursor_in_selection_mut(&mut self, cursor: Cursor) -> Option<(Point, &mut Selection)> {
        self.selected_region.as_mut().and_then(|selected_region| {
            cursor.position().and_then(|cursor_pos| {
                selected_region
                    .normalize()
                    .contains(cursor_pos)
                    .then_some((cursor_pos, selected_region))
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
        selection.selection_status = moving_selection;
        self.selected_region = Some(Selection::new(create_selection_at));
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, other: Point) {
        self.selected_region = self.selected_region.take().map(|selected_region| {
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

        if let Some(selected_region) = self.selected_region {
            selected_region.render_border(&mut frame);
            selected_region.corners().render_circles(&mut frame);
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        self.selected_region
            .and_then(|region| {
                cursor.position().and_then(|cursor_position| {
                    region
                        .corners()
                        .side_at(cursor_position)
                        .map(Side::mouse_icon)
                })
            })
            .unwrap_or_else(|| {
                let is_left_released = state.is_left_released();
                let is_moving_selection = self
                    .selected_region
                    .is_some_and(|selected_region| selected_region.selection_status.is_dragged());

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
                Message::LeftMouseUp
            },
            Mouse(mouse::Event::CursorMoved { position })
                if state.is_left_clicked()
                    && self
                        .selected_region
                        .is_some_and(super::selection::Selection::is_resized) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Resized {
                    resize_side,
                    initial_rect,
                    initial_cursor_pos,
                } = self
                    .selected_region
                    .expect("has `.is_some()` guard")
                    .selection_status
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
                        .selected_region
                        .is_some_and(super::selection::Selection::is_dragged) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Dragged {
                    initial_rect_pos,
                    initial_cursor_pos,
                } = self
                    .selected_region
                    .expect("has `.is_some()` guard")
                    .selection_status
                else {
                    unreachable!();
                };

                Message::MovingSelection {
                    current_cursor_pos: *position,
                    initial_cursor_pos,
                    current_selection: self.selected_region.expect("has `.is_some()` guard"),
                    initial_rect_pos,
                }
            },
            Mouse(mouse::Event::CursorMoved { position })
                if state.is_left_clicked()
                    && self
                        .selected_region
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
