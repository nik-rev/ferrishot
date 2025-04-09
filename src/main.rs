#![cfg_attr(doc, doc = include_str!("../README.md"))]

use iced::{
    Element,
    widget::{button, text},
};

mod screenshot;

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    button(text(counter.value))
        .on_press(Message::Increment)
        .into()
}

fn main() -> iced::Result {
    iced::run("Hello World", update, view)
}
