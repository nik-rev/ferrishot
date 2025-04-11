use iced::keyboard::{Key, Modifiers};
use iced::mouse::{Cursor, Interaction};
use iced::widget::{self, Action, canvas, stack};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};

/// Radius of the 4 corners of the selection
pub const CORNER_RADIUS: f32 = 6.;
/// Color of the selection stroke and corners
pub const SELECTION_COLOR: Color = Color::WHITE;
/// The area around each side which allows that side to be hovered over and
/// resized
pub const INTERACTION_AREA: f32 = 200.;
pub const STROKE_SIZE: f32 = 2.;

use crate::image_renderer::BackgroundImage;
use crate::rectangle::RectangleExt;
use crate::selection::{Selection, SelectionStatus};

/// Holds the state for Groxshot
#[derive(Debug)]
pub struct App {
    /// The full screenshot of the monitor from which groxshot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    screenshot: widget::image::Handle,
    /// Area of the screen that is selected for capture
    selected_region: Option<Selection>,
}

/// Holds information about the mouse
#[derive(Default, Debug, Clone, Copy)]
pub struct MouseState {
    /// Left mouse click is currently being held down
    is_left_down: bool,
}

impl MouseState {
    /// Register a left mouse click
    pub const fn left_click(&mut self) {
        self.is_left_down = true;
    }

    /// Left mouse button
    pub const fn left_release(&mut self) {
        self.is_left_down = false;
    }

    /// If the left mouse button is clicked
    pub const fn is_left_clicked(self) -> bool {
        self.is_left_down
    }

    /// If the left mouse button is released
    pub const fn is_left_released(self) -> bool {
        !self.is_left_down
    }
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
                if let Some((cursor, _side, rect)) = cursor.position().and_then(|cursor_pos| {
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
                    };
                    log::info!("Starting to dragging the selection: {resized:?}");
                    rect.selection_status = resized;
                } else if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor)
                {
                    let dragged = SelectionStatus::Dragged {
                        initial_rect_pos: selected_region.pos(),
                        initial_cursor_pos: cursor,
                    };
                    log::info!("Starting to dragging the selection: {dragged:?}");
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
                    log::info!("Selected initial region at {cursor_position}");
                }
            },
            Message::LeftMouseUp => {
                if let Some(selection) = self.selected_region.as_mut() {
                    selection.selection_status = SelectionStatus::Idle;
                }
            },
            Message::LeftMouseDrag(new_mouse_position) => {
                if let Some((
                    SelectionStatus::Dragged {
                        initial_rect_pos: rect_position,
                        initial_cursor_pos: cursor,
                    },
                    selected_region,
                )) = self
                    .selected_region
                    .map(|region| (region.selection_status, region))
                {
                    self.selected_region = Some(
                        selected_region.set_pos(rect_position + (new_mouse_position - cursor)),
                    );

                    log::debug!("Dragged. New region: {:?}", self.selected_region);
                } else {
                    log::debug!("Updated selection: {new_mouse_position:?}");
                    self.update_selection(new_mouse_position);
                }
            },
            Message::CopyToClipboard => todo!(),
            Message::SaveScreenshot => todo!(),
            Message::Resize(cursor_pos, side) => {
                let Some(selected_region) = self.selected_region.as_mut() else {
                    return ().into();
                };

                // TODO: this can be available in the Resize message
                // No need to handle it here
                let SelectionStatus::Resized {
                    initial_rect,
                    initial_cursor_pos,
                } = selected_region.selection_status
                else {
                    return ().into();
                };

                let dy = cursor_pos.y - initial_cursor_pos.y;
                let dx = cursor_pos.x - initial_cursor_pos.x;

                match side {
                    Side::TopLeft => todo!(),
                    Side::TopRight => todo!(),
                    Side::BottomLeft => todo!(),
                    Side::BottomRight => todo!(),
                    Side::Top => {
                        selected_region.rect =
                            initial_rect.with_height(|h| h - dy).with_y(|y| y + dy);
                    },
                    Side::Right => selected_region.rect = initial_rect.with_width(|w| w + dx),
                    Side::Bottom => {
                        selected_region.rect = initial_rect.with_height(|h| h + dy);
                    },
                    Side::Left => {
                        selected_region.rect =
                            initial_rect.with_width(|w| w - dx).with_x(|x| x + dx);
                    },
                }
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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Exits the application
    Exit,
    /// The left mouse button is down
    LeftMouseDown(Cursor),
    /// The left mouse button is up
    LeftMouseUp,
    /// Left mouse is held down and dragged
    ///
    /// Contains the new point of the mouse
    LeftMouseDrag(Point),
    /// Copy the screenshot to the clipboard
    CopyToClipboard,
    /// Save the screenshot as an image
    SaveScreenshot,
    Resize(Point, Side),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    pub const fn mouse_icon(self) -> mouse::Interaction {
        match self {
            Self::Top | Self::Bottom => mouse::Interaction::ResizingVertically,
            Self::Right | Self::Left => mouse::Interaction::ResizingHorizontally,
            Self::TopLeft | Self::BottomRight => mouse::Interaction::ResizingDiagonallyDown,
            Self::BottomLeft | Self::TopRight => mouse::Interaction::ResizingDiagonallyUp,
        }
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

        let resize_area_intersection_side = self.selected_region.and_then(|selected_region| {
            cursor
                .position()
                .and_then(|cursor_position| selected_region.corners().side_at(cursor_position))
        });

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
                    && resize_area_intersection_side.is_some()
                    && self
                        .selected_region
                        .is_none_or(super::selection::Selection::is_resized) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let cursor_side = resize_area_intersection_side.expect("has `.is_some()` guard");
                Message::Resize(*position, cursor_side)
            },
            Mouse(mouse::Event::CursorMoved { position }) if state.is_left_clicked() => {
                Message::LeftMouseDrag(*position)
            },
            _ => return None,
        };

        // dbg!(
        //     state.is_left_clicked(),
        //     resize_area_intersection_side,
        //     self.selected_region,
        //     &message,
        //     cursor.position(),
        //     self.selected_region.map(|x| x.corners())
        // );

        Some(Action::publish(message))
    }
}
