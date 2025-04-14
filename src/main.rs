//! The ferrishot app

use ferrishot::App;
use iced::Theme;

fn main() {
    ferrishot::initialize_logging();

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
            ferrishot::run_clipboard_daemon().expect("Failed to run clipboard daemon");
            return;
        }
    }

    iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            level: iced::window::Level::Normal,
            fullscreen: true,
            ..Default::default()
        })
        .subscription(|_state| iced::keyboard::on_key_press(App::handle_key_press))
        .title("ferrishot")
        .theme(|_| {
            Theme::custom_with_fn(
                "ferrishot".to_string(),
                iced::theme::Palette {
                    primary: ferrishot::CONFIG.accent_color.into(),
                    ..iced::theme::Palette::LIGHT
                },
                |palette| iced::theme::palette::Extended {
                    primary: iced::theme::palette::Primary {
                        base: iced::theme::palette::Pair {
                            color: iced::Color::BLACK,
                            text: ferrishot::foreground_for(palette.primary),
                        },
                        ..iced::theme::palette::EXTENDED_LIGHT.primary
                    },
                    ..*iced::theme::palette::EXTENDED_LIGHT
                },
            )
        })
        .run()
        .expect("Failed to start ferrishot");

    // open file explorer to choose where to save the image
    if let Some(saved_image) = ferrishot::SAVED_IMAGE.get() {
        // NOTE: The file dialog can be closed by the user, so it is
        // not an error if we can't get the path for one reason or another
        if let Some(save_path) = rfd::FileDialog::new()
            .set_title("Save Screenshot")
            .save_file()
        {
            saved_image
                .save(save_path)
                .expect("Failed to save the image");
        }
    }
}
