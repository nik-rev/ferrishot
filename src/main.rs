#![doc = include_str!("../README.md")]

use ferrishot::App;

fn main() {
    // On linux, a daemon is required to provide clipboard access even when
    // the process dies.
    //
    // More info: <https://docs.rs/arboard/3.5.0/arboard/trait.SetExtLinux.html#tymethod.wait>
    #[cfg(target_os = "linux")]
    {
        if std::env::args()
            .nth(1)
            .as_deref()
            .is_some_and(|arg| arg == ferrishot::CLIPBOARD_DAEMON_ID)
        {
            ferrishot::run_clipboard_daemon().unwrap();
            return;
        }
    }

    env_logger::builder().format_timestamp(None).init();

    let _ = iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            level: iced::window::Level::Normal,
            fullscreen: true,
            ..Default::default()
        })
        .subscription(|_state| iced::keyboard::on_key_press(App::handle_key_press))
        .title("ferrishot")
        .run();
}
