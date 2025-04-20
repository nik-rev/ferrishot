//! The ferrishot app

use miette::miette;
use std::sync::LazyLock;

use ferrishot::{App, CLI};
use iced::Font;
use miette::IntoDiagnostic;

/// RGBA bytes for the Logo of ferrishot. Generated with `build.rs`
const LOGO: &[u8; 64 * 64 * 4] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

fn main() -> miette::Result<()> {
    // This will parse the command line arguments.
    //
    // Needs to come before the logging initialization because there
    // is an argument to change the verbosity
    LazyLock::force(&CLI);

    // Initialize logging
    // TODO:
    // - log to file.
    // - add ways to increase logging with command line arguments. Currently you must use `RUST_LOG=info`
    env_logger::builder().init();

    LazyLock::force(&ferrishot::CONFIG);

    if CLI.dump_default_config {
        std::fs::create_dir_all(
            std::path::PathBuf::from(&CLI.config_file)
                .parent()
                .ok_or_else(|| miette!("Could not get parent path of {}", CLI.config_file))?,
        )
        .into_diagnostic()?;
        std::fs::write(&CLI.config_file, ferrishot::DEFAULT_KDL_CONFIG_STR).into_diagnostic()?;

        return Ok(());
    }

    // tray icon for Mac / Windows
    #[cfg(not(target_os = "linux"))]
    {
        let icon =
            tray_icon::Icon::from_rgba(LOGO.to_vec(), 64, 64).expect("Icon to be valid RGBA bytes");

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
            return Ok(());
        }
    }

    iced::application(App::default, App::update, App::view)
        .window(iced::window::Settings {
            level: iced::window::Level::Normal,
            fullscreen: true,
            icon: Some(
                iced::window::icon::from_rgba(LOGO.to_vec(), 64, 64)
                    .expect("Icon to be valid RGBA bytes"),
            ),
            ..Default::default()
        })
        .title("ferrishot")
        .default_font(Font::MONOSPACE)
        .run()
        .map_err(|err| miette!("Failed to start ferrishot: {err}"))?;

    // open file explorer to choose where to save the image
    if let Some(saved_image) = ferrishot::SAVED_IMAGE.get() {
        if let Some(save_path) = rfd::FileDialog::new()
            .set_title("Save Screenshot")
            .save_file()
        {
            saved_image
                .save(save_path)
                .map_err(|err| miette!("Failed to save the screenshot: {err}"))?;
        } else {
            log::info!("The file dialog was closed before a file was chosen");
        }
    }

    Ok(())
}
