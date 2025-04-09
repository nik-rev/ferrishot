#![cfg_attr(doc, doc = include_str!("../README.md"))]

use std::io::Read;

use iced::widget::{self, canvas, column, text};
use iced::{Color, Element, Rectangle, Renderer, Task, Theme, mouse};
use image::EncodableLayout as _;

mod screenshot;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Message {
    /// Exits the application
    Close,
}

struct App {
    radius: f32,
    region: Option<Rect>,
    image_handle: widget::image::Handle,
}

impl App {
    pub fn new(image_handle: widget::image::Handle) -> Self {
        Self {
            image_handle,
            radius: Default::default(),
            region: Default::default(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Default)]
struct Rect {
    top_left: Coordinate,
    top_right: Coordinate,
    bottom_left: Coordinate,
    bottom_right: Coordinate,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Default)]
struct Coordinate {
    x: f32,
    y: f32,
}

#[allow(unused_variables)]
fn update(counter: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Close => iced::exit(),
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
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.draw_image(bounds, &self.image_handle);

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);
        // And fill it with some color
        frame.fill(&circle, Color::BLACK);
        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

fn view(state: &App) -> Element<Message> {
    column![text("lol"), iced::widget::canvas(state)].into()
}

fn main() -> iced::Result {
    iced::application(
        || {
            let image = screenshot::get();
            let handle = iced::widget::image::Handle::from_rgba(
                image.width(),
                image.height(),
                image.into_raw(),
            );
            App::new(handle)
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
