#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::keyboard::Modifiers;
use iced::widget::{self, canvas, container};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Task, Theme, mouse};
use image_renderer::BackgroundImage;

mod image_renderer;
mod screenshot;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    /// Exits the application
    Close,
}

#[derive(Default)]
struct App;

impl App {
    fn view(&self) -> Element<Message> {
        BackgroundImage::default().into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => iced::exit(),
        }
    }
}

#[derive(Default)]
struct CanvasContext {
    left_mouse_down: bool,
    /// Area of the screen that is selected for capture
    selected_region: Option<Rectangle>,
}

impl CanvasContext {
    /// Create an empty selection at the current position
    pub fn create_selection_at(&mut self, create_selection_at: Point) {
        self.selected_region = Some(Rectangle::new(create_selection_at, Size::default()))
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, create_selection_at: Point) {
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
            let width = create_selection_at.x - region.x;
            let height = create_selection_at.y - region.y;
            Rectangle::new(region.position(), Size { width, height })
        });
    }
}

impl<Message> canvas::Program<Message> for App {
    type State = CanvasContext;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // if let Some(selected_region) = state.selected_region {
        frame.fill_rectangle(
            Point { x: 200., y: 200. },
            // selected_region.position(),
            Size {
                width: 75.,
                height: 75.,
            },
            // selected_region.size(),
            Color::from_rgb(1., 0., 0.),
        );

        frame.fill_text("Hello");
        // }

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
                if let Some(selected_region) = state.selected_region {
                    if let Some(_cursor_position_over_selected_region) =
                        cursor.position_over(selected_region)
                    {
                        // move the selection
                    } else {
                        // cursor is not in the selected region
                        // create new selection
                        let cursor_position =
                            cursor.position().expect("cursor to be in the monitor");
                        // no region is selected, select the initial region
                        state.create_selection_at(cursor_position);
                    };
                } else {
                    // no region is selected, select the initial region
                    let cursor_position = cursor.position().expect("cursor to be in the monitor");
                    state.create_selection_at(cursor_position);
                };
            },
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.left_mouse_down = false;
            },
            Mouse(mouse::Event::CursorMoved { position }) => {
                if state.left_mouse_down {
                    state.update_selection(*position);
                    // dragging
                }
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
