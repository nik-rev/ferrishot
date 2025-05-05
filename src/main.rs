//! The ferrishot app

use std::sync::Arc;

use clap::Parser as _;
use ferrishot::Cli;
use miette::IntoDiagnostic as _;
use miette::miette;

use ferrishot::App;

/// RGBA bytes for the Logo of ferrishot. Generated with `build.rs`
const LOGO: &[u8; 64 * 64 * 4] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.bin"));

#[allow(
    clippy::print_stderr,
    clippy::print_stdout,
    reason = "print from `main` is fine"
)]
#[tokio::main]
async fn main() -> miette::Result<()> {
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
    let cli = Arc::new(Cli::parse());

    if cli.markdown_help {
        clap_markdown::print_help_markdown::<ferrishot::Cli>();
        return Ok(());
    }

    if cli.print_log_file_path {
        println!("{}", ferrishot::DEFAULT_LOG_FILE_PATH.display());
        return Ok(());
    }

    // Setup logging
    ferrishot::logging::initialize(&cli);

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

    let cli_save_path = cli.save_path.clone();

    if let Some(delay) = cli.delay {
        println!("Sleeping for {delay:?}...");
        std::thread::sleep(delay);
    }

    // Parse user's `ferrishot.kdl` config file
    let config = Arc::new(ferrishot::Config::parse(&cli.config_file)?);
    let initial_region = if cli.last_region {
        Cli::last_region()?
    } else {
        cli.region
    };

    match (cli.accept_on_select, initial_region) {
        // If we want to do an action as soon as we have a selection,
        // AND we start the app with the selection: Then don't even launch a window.
        //
        // Run in 'headless' mode and perform the action instantly
        (Some(accept_on_select), Some(region)) => {
            if let Some(output) = App::headless(accept_on_select, region, cli.file.as_ref())
                .await
                .map_err(|err| miette!("Failed to start ferrishot (headless): {err}"))?
            {
                print!("{output}");
            }
        }
        // Launch ferrishot app
        _ => {
            iced::application(
                move || App::new(Arc::clone(&cli), Arc::clone(&config), initial_region),
                App::update,
                App::view,
            )
            .subscription(App::subscription)
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
        }
    }

    if let Some(saved_image) = ferrishot::SAVED_IMAGE.get() {
        if let Some(save_path) = cli_save_path.or_else(|| {
            // Open file explorer to choose where to save the image
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
