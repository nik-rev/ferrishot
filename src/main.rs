#![cfg_attr(doc, doc = include_str!("../README.md"))]

use delegate::delegate;
use iced::keyboard::Modifiers;
use iced::mouse::{Cursor, Interaction};
use iced::widget::{self, Action, canvas, stack};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};
use image_renderer::BackgroundImage;

#[derive(Debug, Default, Copy, Clone)]
struct Selection(Rectangle);

impl Selection {
    /// make sure that the top-left corner is ALWAYS in the top left
    /// (it could be that top-left corner is actually on the bottom right,
    /// and we have a negative width and height):
    ///
    ///                           ----------
    ///                           |        |
    ///                           |        | <- height: -3
    ///                           |        |
    /// our "top left" is here -> O---------
    /// even if the width and height is negative
    fn normalize(&self) -> Rectangle {
        let mut rect = self.0;
        if rect.width.is_sign_negative() {
            rect.x = rect.x + rect.width;
            rect.width = rect.width.abs();
        }
        if rect.height.is_sign_negative() {
            rect.y = rect.y + rect.height;
            rect.height = rect.height.abs();
        }
        rect
    }

    pub fn contains(&self, point: Point) -> bool {
        self.normalize().contains(point)
    }

    /// Create selection with a size of zero
    pub fn new(point: Point) -> Self {
        Self(Rectangle::new(point, Size::default()))
    }

    pub fn with_size(&self, size: Size) -> Self {
        Self(Rectangle::new(self.position(), size))
    }

    pub fn with_position(&self, pos: Point) -> Self {
        Self(Rectangle::new(pos, self.size()))
    }

    pub fn rect(&self) -> Rectangle {
        self.0
    }

    /// The x-coordinate of the top left point
    pub fn x(&self) -> f32 {
        self.0.x
    }

    /// The y-coordinate of the top left point
    pub fn y(&self) -> f32 {
        self.0.y
    }

    delegate! {
        to self.0 {
            pub fn position(&self) -> Point;
            pub fn size(&self) -> Size;
        }
    }
}

mod image_renderer;
mod screenshot;

#[derive(Debug, Default)]
struct MovingSelection {
    /// top left point of the selection before we started moving it
    top_left_anchor: Point,
    /// cursor position before we started moving the selection with the cursor
    cursor_anchor: Point,
}

#[derive(Default, Debug)]
struct App {
    /// Left mouse click is currently being held down
    left_mouse_down: bool,
    /// The selection is currently being moved (hold left click + move)
    moving_selection: Option<MovingSelection>,
    /// Area of the screen that is selected for capture
    selected_region: Option<Selection>,
    message: Option<String>,
}

impl App {
    fn view(&self) -> Element<Message> {
        stack![
            BackgroundImage::default(),
            canvas(self).width(Length::Fill).height(Length::Fill),
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => iced::exit(),
        }
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

    /// Create an empty selection at the current position
    pub fn create_selection_at(&mut self, create_selection_at: Point) {
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
enum Message {
    /// Exits the application
    Close,
}

impl canvas::Program<Message> for App {
    type State = App;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        if let Some(selected_region) = state.selected_region {
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
        state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        // when the cursor is inside of the selected region,
        if (!state.left_mouse_down || state.moving_selection.is_some())
            && state.cursor_in_selection(cursor).is_some()
        {
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
        state.message = format!(
            "cursor: {cursor:?}, region: {:?}",
            self.selected_region.unwrap_or_default()
        )
        .into();
        use iced::Event::Mouse;
        match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.left_mouse_down = true;
                if let Some((cursor, selected_region)) = state.cursor_in_selection(cursor) {
                    state.moving_selection = Some(MovingSelection {
                        top_left_anchor: selected_region.position(),
                        cursor_anchor: cursor,
                    });
                    return Some(Action::request_redraw());
                };

                // no region is selected, select the initial region
                let cursor_position = cursor.position().expect("cursor to be in the monitor");
                state.create_selection_at(cursor_position);

                return Some(Action::request_redraw());
            },
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.left_mouse_down = false;
                state.moving_selection = None;
            },
            Mouse(mouse::Event::CursorMoved { position }) if state.left_mouse_down => {
                if let Some(MovingSelection {
                    top_left_anchor,
                    cursor_anchor,
                }) = state.moving_selection
                {
                    if let Some(Rectangle { width, height, .. }) =
                        state.selected_region.map(|r| r.rect())
                    {
                        let Point { x, y } = top_left_anchor + (*position - cursor_anchor);

                        state.selected_region = Some(
                            Selection::default()
                                .with_size(Size { width, height })
                                .with_position(Point { x, y }),
                        );
                    }
                } else {
                    state.update_selection(*position);
                }
                return Some(Action::request_redraw());
            },
            _ => (),
        };

        None
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            level: iced::window::Level::AlwaysOnTop,
            fullscreen: true,
            ..Default::default()
        })
        .subscription(|_state| {
            iced::keyboard::on_key_press(|key, mods| {
                use iced::keyboard::Key;
                match (key, mods) {
                    (Key::Named(iced::keyboard::key::Named::Escape), _) => Some(Message::Close),
                    (Key::Character(str @ _), Modifiers::CTRL) if str == "y" => {
                        // save screenshot to path
                        None
                    },
                    (Key::Character(str @ _), Modifiers::CTRL) if str == "c" => {
                        // copy to clipboard
                        None
                    },
                    _ => None,
                }
            })
        })
        .run()
}
