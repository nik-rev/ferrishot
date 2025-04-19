//! The ferrishot app

use std::io::Read as _;

use ferrishot::{App, CONFIG, Config};
use iced::Font;
use miette::IntoDiagnostic as _;

/// Logo of ferrishot
const LOGO: &[u8; 0x4000] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

fn main() -> miette::Result<()> {
    let mut buf = String::new();

    std::io::stdin()
        .read_to_string(&mut buf)
        .into_diagnostic()?;

    let config = knus::parse::<Config>("<stdin>", &buf)?;

    let item = &config.settings;
    dbg!(&config);

    Ok(())
}

fn maint() {
    env_logger::builder().init();

    // tray icon for Mac / Windows
    #[cfg(not(target_os = "linux"))]
    {
        let icon = tray_icon::Icon::from_rgba(LOGO.to_vec(), 64, 64).expect("Failed to open icon");

        let _tray_icon = tray_icon::TrayIconBuilder::new()
            .with_title("ferrishot")
            .with_tooltip("Take a screenshot using ferrishot")
            .with_icon(icon)
            .build();
    }

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
            icon: Some(
                iced::window::icon::from_rgba(LOGO.to_vec(), 64, 64)
                    .expect("logo.bin contains valid RGBA"),
            ),
            ..Default::default()
        })
        .title("ferrishot")
        .default_font(Font::MONOSPACE)
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
