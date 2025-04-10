use iced::keyboard::{Key, Modifiers};
use iced::mouse::{Cursor, Interaction};
use iced::widget::{self, Action, canvas, stack};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};

use crate::image_renderer::BackgroundImage;
use crate::selection::{Selection, SelectionStatus};

#[derive(Debug)]
pub struct App {
    bg: widget::image::Handle,
    /// Left mouse click is currently being held down
    left_mouse_down: bool,
    /// Area of the screen that is selected for capture
    selected_region: Option<Selection>,
}

impl Default for App {
    fn default() -> Self {
        let screenshot = crate::screenshot::screenshot().unwrap();
        Self {
            bg: screenshot,
            left_mouse_down: false,
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
            BackgroundImage::new(self.bg.clone()),
            canvas(self).width(Length::Fill).height(Length::Fill),
        ]
        .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => return iced::exit(),
            Message::LeftMouseDown(cursor) => {
                self.left_mouse_down = true;
                if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor) {
                    let status = SelectionStatus::Dragged {
                        rect_position: selected_region.position(),
                        cursor,
                    };
                    log::info!("Dragging the selection: {status:?}");
                    selected_region.moving_selection = Some(status);
                } else {
                    // no region is selected, select the initial region
                    let cursor_position = cursor.position().expect("cursor to be in the monitor");
                    self.create_selection_at(
                        cursor_position,
                        self.selected_region
                            .and_then(|region| region.moving_selection),
                    );
                    log::info!("Selected initial region at {cursor_position}");
                };
            },
            Message::LeftMouseUp => {
                self.left_mouse_down = false;
                if let Some(selection) = self.selected_region.as_mut() {
                    selection.moving_selection = None;
                }
            },
            Message::LeftMouseDrag(new_mouse_position) => {
                if let Some((
                    SelectionStatus::Dragged {
                        rect_position,
                        cursor,
                    },
                    selected_region,
                )) = self.selected_region.and_then(|region| {
                    region
                        .moving_selection
                        .map(|moving_selection| (moving_selection, region))
                }) {
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
        moving_selection: Option<SelectionStatus>,
    ) {
        let mut selection = Selection::new(create_selection_at);
        if let Some(moving_selection) = moving_selection {
            selection.moving_selection = Some(moving_selection)
        }
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
}

impl canvas::Program<Message> for App {
    type State = ();

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
            frame.fill_rectangle(
                selected_region.position(),
                selected_region.size(),
                Color::BLACK,
            );
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        // when the cursor is inside of the selected region,
        if (!self.left_mouse_down
            || self
                .selected_region
                .is_some_and(|selected_region| selected_region.moving_selection.is_some()))
            && self.cursor_in_selection(cursor).is_some()
        {
            Interaction::Grab
        } else {
            Interaction::Crosshair
        }
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        use iced::Event::Mouse;
        let message = match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                Message::LeftMouseDown(cursor)
            },
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => Message::LeftMouseUp,
            Mouse(mouse::Event::CursorMoved { position }) if self.left_mouse_down => {
                Message::LeftMouseDrag(*position)
            },
            _ => return None,
        };

        Some(Action::publish(message))
    }
}
