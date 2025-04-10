#![cfg_attr(doc, doc = include_str!("../README.md"))]

use app::{App, Message};
use iced::keyboard::Modifiers;

mod app;
mod image_renderer;
mod screenshot;
mod selection;

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

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
                    (Key::Named(iced::keyboard::key::Named::Escape), _) => Some(Message::Exit),
                    (Key::Character(ch), Modifiers::CTRL) if ch == "c" => {
                        Some(Message::CopyToClipboard)
                    },
                    (Key::Character(ch), Modifiers::CTRL) if ch == "s" => {
                        Some(Message::SaveScreenshot)
                    },
                    _ => None,
                }
            })
        })
        .run()
}
