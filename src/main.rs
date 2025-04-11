#![doc = include_str!("../README.md")]

use ferrishot::App;

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            level: iced::window::Level::AlwaysOnTop,
            fullscreen: true,
            ..Default::default()
        })
        .subscription(|_state| iced::keyboard::on_key_press(App::handle_key_press))
        .run()
}
