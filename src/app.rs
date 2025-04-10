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
pub const INTERACTION_AREA: f32 = 20.;
pub const STROKE_SIZE: f32 = 2.;

use crate::image_renderer::BackgroundImage;
use crate::selection::{Selection, SelectionStatus};

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
    pub fn left_click(&mut self) {
        self.is_left_down = true
    }

    /// Left mouse button
    pub fn left_release(&mut self) {
        self.is_left_down = false
    }

    /// If the left mouse button is clicked
    pub fn is_left_clicked(&self) -> bool {
        self.is_left_down
    }

    /// If the left mouse button is released
    pub fn is_left_released(&self) -> bool {
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
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => return iced::exit(),
            Message::LeftMouseDown(cursor) => {
                // if let Some((cursor, _side, rect)) = cursor.position().and_then(|cursor_pos|
                // {     self.selected_region.as_mut().map(|selected_region| {
                //         (
                //             cursor_pos,
                //             selected_region.corners().side_at(cursor_pos),
                //             selected_region,
                //         )
                //     })
                // }) {
                //     let resized = SelectionStatus::Resized {
                //         rect: rect.rect,
                //         cursor,
                //     };
                //     log::info!("Starting to dragging the selection: {resized:?}");
                //     rect.selection_status = resized;
                // } else

                if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor) {
                    let dragged = SelectionStatus::Dragged {
                        rect_position: selected_region.position(),
                        cursor,
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
                };
            },
            Message::LeftMouseUp => {
                if let Some(selection) = self.selected_region.as_mut() {
                    selection.selection_status = SelectionStatus::Idle;
                }
            },
            Message::LeftMouseDrag(new_mouse_position) => {
                if let Some((
                    SelectionStatus::Dragged {
                        rect_position,
                        cursor,
                    },
                    selected_region,
                )) = self
                    .selected_region
                    .map(|region| (region.selection_status, region))
                {
                    self.selected_region = Some(
                        selected_region
                            .with_position(rect_position + (new_mouse_position - cursor)),
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
                match side {
                    Side::TopLeft => todo!(),
                    Side::TopRight => todo!(),
                    Side::BottomLeft => todo!(),
                    Side::BottomRight => todo!(),
                    Side::Top => todo!(),
                    Side::Right => todo!(),
                    Side::Bottom => todo!(),
                    Side::Left => todo!(),
                }
            },
            _ => todo!(),
        };

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
        self.selected_region = Some(Selection::new(create_selection_at))
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
                .with_position(selected_region.position())
                .with_size(Size { width, height })
        });
    }
}

#[derive(Debug, Clone)]
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
pub enum Resize {
    /// Increase size of the selection
    Expand,
    /// Decreases size of the selection
    Shrink,
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
    pub fn mouse_icon(&self) -> mouse::Interaction {
        match self {
            Side::Top | Side::Bottom => mouse::Interaction::ResizingVertically,
            Side::Right | Side::Left => mouse::Interaction::ResizingHorizontally,
            Side::TopLeft | Side::BottomRight => mouse::Interaction::ResizingDiagonallyDown,
            Side::BottomLeft | Side::TopRight => mouse::Interaction::ResizingDiagonallyUp,
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
        if let Some(mouse_icon) = self.selected_region.and_then(|region| {
            cursor.position().and_then(|cursor_position| {
                region
                    .corners()
                    .side_at(cursor_position)
                    .map(|corners| corners.mouse_icon())
            })
        }) {
            mouse_icon
        }
        // when the cursor is inside of the selected region,
        else if {
            let is_left_released = state.is_left_released();
            let is_moving_selection = self
                .selected_region
                .is_some_and(|selected_region| selected_region.selection_status.is_dragged());

            (is_left_released || is_moving_selection) && self.cursor_in_selection(cursor).is_some()
        } {
            Interaction::Grab
        } else {
            Interaction::Crosshair
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        use iced::Event::Mouse;

        // let cursor_side = self.selected_region.and_then(|selected_region| {
        //     cursor
        //         .position()
        //         .and_then(|cursor_position|
        // selected_region.corners().side_at(cursor_position)) });

        let message = match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.left_click();
                Message::LeftMouseDown(cursor)
            },
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.left_release();
                Message::LeftMouseUp
            },
            // Mouse(mouse::Event::CursorMoved { position })
            //     if state.is_left_clicked()
            //         && cursor_side.is_some()
            //         && self
            //             .selected_region
            //             .is_none_or(|selection_region| !selection_region.is_dragged()) =>
            // {
            //     // FIXME: this will not be necessary when we have `let_chains`
            //     let cursor_side = cursor_side.expect("has `.is_some()` guard");
            //     Message::Resize(*position, cursor_side)
            // },
            Mouse(mouse::Event::CursorMoved { position }) if state.is_left_clicked() => {
                Message::LeftMouseDrag(*position)
            },
            _ => return None,
        };

        Some(Action::publish(message))
    }
}
