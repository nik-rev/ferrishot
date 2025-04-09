#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::widget::{self, canvas};
use iced::{Element, Length, Rectangle, Renderer, Task, Theme, mouse};

mod screenshot;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Message {
    /// Exits the application
    Close,
}

#[allow(unused_variables)]
fn update(counter: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Close => iced::exit(),
    }
}

struct App {
    image_handle: widget::image::Handle,
}

impl App {
    pub fn new(image_handle: widget::image::Handle) -> Self {
        Self { image_handle }
    }
}

impl<Message> canvas::Program<Message> for App {
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

        let img = canvas::Image::new(self.image_handle.clone())
            // this is necessary otherwise the rendered image is going to be blurry
            .filter_method(widget::image::FilterMethod::Nearest);

        frame.draw_image(bounds, img);

        vec![frame.into_geometry()]
    }
}

fn view(state: &App) -> Element<Message> {
    iced::widget::canvas(state)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application(
        || {
            let screenshot = screenshot::screenshot().unwrap();
            App::new(screenshot)
        },
        update,
        view,
    )
    .window(iced::window::Settings {
        level: iced::window::Level::AlwaysOnTop,
        fullscreen: true,
        ..Default::default()
    })
    .subscription(|_state| {
        iced::keyboard::on_key_press(|key, _mods| {
            match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                    Some(Message::Close)
                },
                _ => None,
            }
        })
    })
    .run()
}
