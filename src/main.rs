#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::advanced::debug::core::SmolStr;
use iced::keyboard::Modifiers;
use iced::widget::{self, canvas};
use iced::{Element, Length, Point, Rectangle, Renderer, Task, Theme, mouse};

mod screenshot;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    /// Exits the application
    Close,
    Click(Point),
}

struct App {
    image_handle: widget::image::Handle,
}

impl App {
    fn view(&self) -> Element<Message> {
        iced::widget::canvas(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => iced::exit(),
            Message::Click(point) => {
                todo!()
            },
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let screenshot = screenshot::screenshot().unwrap();
        Self {
            image_handle: screenshot,
        }
    }
}

#[derive(Default)]
struct CanvasContext {
    left_mouse_down: bool,
    /// Area of the screen that is selected for capture
    selected_region: Rectangle,
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

        let img = canvas::Image::new(self.image_handle.clone())
            // this is necessary otherwise the rendered image is going to be blurry
            .filter_method(widget::image::FilterMethod::Nearest);

        frame.draw_image(bounds, img);

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.left_mouse_down = true;
                None
            },
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.left_mouse_down = false;
                None
            },
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.left_mouse_down {
                    // dragging
                }
                None
            },
            _ => None,
        }
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
                match (key, mods) {
                    (iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape), _) => {
                        Some(Message::Close)
                    },
                    (iced::keyboard::Key::Character(str @ _), Modifiers::CTRL) if str == "y" => {
                        // save path
                        None
                    },
                    (iced::keyboard::Key::Character(str @ _), Modifiers::CTRL) if str == "c" => {
                        // copy to clipboard
                        None
                    },
                    _ => None,
                }
            })
        })
        .run()
}
