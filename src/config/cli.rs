//! Parse the command line arguments passed to ferrishot
use std::time::Duration;
use std::{path::PathBuf, sync::LazyLock};

use clap::{Parser, ValueHint};
use etcetera::BaseStrategy as _;
use iced::Rectangle;
use indoc::indoc;

use crate::geometry::RectangleExt as _;

use anstyle::{AnsiColor, Effects};

/// get CLI styles
fn get_cli_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
        .valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
}

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco", styles = get_cli_styles())]
#[expect(clippy::struct_excessive_bools, reason = "normal for CLIs")]
pub struct Cli {
    /// Instead of taking a screenshot of the desktop, open this image instead
    //
    // NOTE: Currently disabled because if the screenshot is not the same size as the desktop,
    // it will cause bugs as we consider 0,0 in the Canvas to be the origin but it is not necessarily,
    // when the desktop and the image are not the same size
    //
    // TODO: Fix this argument
    //
    #[arg(hide = true, value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Open with a region pre-selected
    ///
    /// Format: `<width>x<height>+<top-left-x>+<top-left-y>`
    #[arg(short, long, value_parser = Rectangle::from_str, value_name = "WxH+X+Y", verbatim_doc_comment)]
    pub region: Option<Rectangle>,

    /// Use last region
    #[arg(short, long, conflicts_with = "region")]
    pub last_region: bool,

    /// Accept on first selection
    #[arg(
        short,
        long,
        value_name = "ACTION",
        long_help = indoc! {
            "
            Accept capture and perform the action as soon as a selection is made

            If holding `ctrl` while you are releasing the left mouse button on the first selection,
            the behavior is cancelled

            It's quite useful to run ferrishot, select a region and have it instantly be copied to the
            clipboard for example. In 90% of situations you won't want to do much post-processing of
            the region and this makes that experience twice as fast. You can always opt-out with `ctrl`

            Using this option with `--region` or `--last-region` will run ferrishot in 'headless mode',
            without making a new window."
        }
    )]
    pub accept_on_select: Option<crate::image::action::Message>,

    /// Wait this long before launch
    #[arg(
        short,
        long,
        value_name = "MILLISECONDS",
        value_parser = |s: &str| s.parse().map(Duration::from_millis),
    )]
    pub delay: Option<Duration>,

    /// Save image to path
    #[arg(
        short,
        long,
        value_name = "PATH",
        long_help = "Instead of opening a file picker to save the screenshot, save it to this path instead",
        value_hint = ValueHint::FilePath
    )]
    pub save_path: Option<PathBuf>,

    //
    // --- Config ---
    //
    /// Dump default config
    #[arg(
        help_heading = "Config",
        short = 'D',
        long,
        help = format!("Write the default config to {}",  DEFAULT_CONFIG_FILE_PATH.display()),
        long_help = format!("Write contents of the default config to {}", DEFAULT_CONFIG_FILE_PATH.display()),
    )]
    pub dump_default_config: bool,

    /// Use the provided config file
    #[arg(
        help_heading = "Config",
        short = 'C',
        long,
        value_name = "file.kdl",
        default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string_lossy().to_string(),
        value_hint = ValueHint::FilePath
    )]
    pub config_file: String,

    //
    // --- Output
    //
    /// Run in silent mode
    #[arg(
        help_heading = "Output",
        short = 'S',
        long,
        long_help = "Run in silent mode. Do not print anything"
    )]
    pub silent: bool,
    /// Print in JSON format
    #[arg(help_heading = "Output", short, long, conflicts_with = "silent")]
    pub json: bool,

    //
    // --- Debug ---
    //
    // These options are hidden from the user
    //
    /// Choose a minumum level at which to log
    #[arg(help_heading = "Debug", long, hide = true, default_value_t = log::LevelFilter::Error)]
    pub log_level: log::LevelFilter,
    /// Log to stdout instead of file
    #[arg(help_heading = "Debug", long, hide = true, conflicts_with = "silent")]
    pub log_stdout: bool,
    /// Path to the log file
    #[arg(
        help_heading = "Debug",
        long,
        hide = true,
        default_value_t = DEFAULT_LOG_FILE_PATH.to_string_lossy().to_string(),
        value_hint = ValueHint::FilePath
    )]
    pub log_file: String,
    /// Launch ferrishot in debug mode (F12)
    #[arg(help_heading = "Debug", long, hide = true)]
    pub debug: bool,
    /// Output the path to the log file
    #[arg(help_heading = "Debug", long, hide = true, conflicts_with = "silent")]
    pub print_log_file_path: bool,
}

/// Represents the default location of the config file
static DEFAULT_CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    etcetera::choose_base_strategy().map_or_else(
        |err| {
            log::warn!("Could not determine the config directory: {err}");
            PathBuf::from("ferrishot.kdl")
        },
        |strategy| strategy.config_dir().join("ferrishot.kdl"),
    )
});

/// Represents the default location of the config file
pub static DEFAULT_LOG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    etcetera::choose_base_strategy().map_or_else(
        |err| {
            log::warn!("Could not determine the config directory: {err}");
            PathBuf::from("ferrishot.log")
        },
        |strategy| strategy.cache_dir().join("ferrishot.log"),
    )
});
