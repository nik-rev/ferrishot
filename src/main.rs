#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::{Color, Element, Rectangle, Renderer, Theme, mouse, widget::canvas};

mod screenshot;

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Circle {
    radius: f32,
}

#[allow(unused_variables)]
fn update(counter: &mut Circle, message: Message) {}

impl<Message> canvas::Program<Message> for Circle {
    type State = ();
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);
        // And fill it with some color
        frame.fill(&circle, Color::BLACK);
        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

fn view(_state: &Circle) -> Element<Message> {
    iced::widget::canvas(Circle { radius: 50.0 }).into()
}

fn main() -> iced::Result {
    iced::application(|| Circle::default(), update, view)
        .window(iced::window::Settings {
            level: iced::window::Level::AlwaysOnTop,
            fullscreen: true,
            ..Default::default()
        })
        .run()
}
