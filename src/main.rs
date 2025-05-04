//! The ferrishot app

use std::sync::Arc;

use clap::Parser;
use miette::IntoDiagnostic as _;
use miette::miette;

use ferrishot::App;

/// RGBA bytes for the Logo of ferrishot. Generated with `build.rs`
const LOGO: &[u8; 64 * 64 * 4] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

fn main() -> miette::Result<()> {
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

    // Parse command line arguments
    let cli = Arc::new(ferrishot::Cli::parse());

    if cli.print_log_file_path {
        println!("{}", ferrishot::DEFAULT_LOG_FILE_PATH.display());
        return Ok(());
    }

    // Setup logging
    ferrishot::logging::initialize(&cli);

    let save_path = cli.save_path.clone();

    if cli.dump_default_config {
        std::fs::create_dir_all(
            std::path::PathBuf::from(&cli.config_file)
                .parent()
                .ok_or_else(|| miette!("Could not get parent path of {}", cli.config_file))?,
        )
        .into_diagnostic()?;

        std::fs::write(&cli.config_file, ferrishot::DEFAULT_KDL_CONFIG_STR).into_diagnostic()?;

        println!("Wrote the default config file to {}", cli.config_file);

        return Ok(());
    }

    if let Some(delay) = cli.delay {
        println!("Sleeping for {delay:?}...");
        std::thread::sleep(delay);
    }

    // Parse user's `ferrishot.kdl` config file
    let config = Arc::new(ferrishot::Config::parse(&cli.config_file)?);

    // Launch ferrishot
    iced::application(
        move || App::new(Arc::clone(&cli), Arc::clone(&config)),
        App::update,
        App::view,
    )
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
    .default_font(iced::Font::MONOSPACE)
    .run()
    .map_err(|err| miette!("Failed to start ferrishot: {err}"))?;

    if let Some(saved_image) = ferrishot::SAVED_IMAGE.get() {
        // Open file explorer to choose where to save the image

        if let Some(save_path) = save_path.or_else(|| {
            let dialog = rfd::FileDialog::new()
                .set_title("Save Screenshot")
                .save_file();

            if dialog.is_none() {
                log::info!("The file dialog was closed before a file was chosen");
            }

            dialog
        }) {
            saved_image
                .save(save_path)
                .map_err(|err| miette!("Failed to save the screenshot: {err}"))?;
        }
    }

    Ok(())
}
