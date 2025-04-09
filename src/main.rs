#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::keyboard::Modifiers;
use iced::mouse::Cursor;
use iced::widget::{self, Action, canvas, stack};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};
use image_renderer::BackgroundImage;

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
    selected_region: Option<Rectangle>,
}

impl App {
    fn view(&self) -> Element<Message> {
        stack![
            BackgroundImage::default(),
            canvas(self).width(Length::Fill).height(Length::Fill)
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
    fn point_in_region(&self, cursor: Cursor) -> Option<(Point, Rectangle)> {
        self.selected_region.and_then(|selected_region| {
            cursor
                .position_over(selected_region)
                .map(|cursor_anchor| (cursor_anchor, selected_region))
        })
    }

    /// Create an empty selection at the current position
    pub fn create_selection_at(&mut self, create_selection_at: Point) {
        self.selected_region = Some(Rectangle::new(create_selection_at, Size::default()))
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, other: Point) {
        self.selected_region = self.selected_region.take().map(|region| {
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
            let width = other.x - region.x;
            let height = other.y - region.y;
            Rectangle::new(region.position(), Size { width, height })
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

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        use iced::Event::Mouse;
        match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.left_mouse_down = true;
                if let Some((cursor, selected_region)) = state.point_in_region(cursor) {
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
                    if let Some(Rectangle { width, height, .. }) = state.selected_region {
                        let Point { x, y } = top_left_anchor + (*position - cursor_anchor);

                        state.selected_region = Some(Rectangle {
                            x,
                            y,
                            width,
                            height,
                        })
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
