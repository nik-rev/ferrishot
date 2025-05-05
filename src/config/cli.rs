//! Parse the command line arguments passed to ferrishot
use std::time::Duration;
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy as _;
use iced::Rectangle;

use crate::rect::RectangleExt as _;

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
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
    #[arg(hide = true)]
    pub file: Option<PathBuf>,

    /// Open ferrishot with a region pre-selected
    ///
    /// Using this option with `--accept-on-select` will run ferrishot in 'headless mode',
    /// without making a new window
    ///
    /// Format: `<width>x<height>+<top-left-x>+<top-left-y>`
    #[arg(long, value_parser = Rectangle::from_str, value_name = "WxH+X+Y", verbatim_doc_comment)]
    pub region: Option<Rectangle>,
    /// Open ferrishot with the last selected region
    #[arg(long, conflicts_with = "region")]
    pub last_region: bool,

    /// Accept capture and perform the action as soon as a selection is made
    ///
    /// If holding `ctrl` while you are releasing the left mouse button on the first selection,
    /// the behaviour is cancelled
    ///
    /// It's quite useful to run ferrishot, select a region and have it instantly be copied to the
    /// clipboard for example. In 90% of situations you won't want to do much post-processing of
    /// the region and this makes that experience twice as fast. You can always opt-out with `ctrl`
    #[arg(short('a'), long, value_name = "ACTION", verbatim_doc_comment)]
    pub accept_on_select: Option<crate::image::action::Message>,

    /// Wait this long before launch
    #[arg(
        long,
        value_name = "MILLISECONDS",
        value_parser = |s: &str| s.parse().map(Duration::from_millis),
    )]
    pub delay: Option<Duration>,

    /// Instead of opening a file picker to save the screenshot, save it to this path instead
    #[arg(long, value_name = "PATH")]
    pub save_path: Option<PathBuf>,

    //
    // --- Config ---
    //
    /// Write the default config file
    #[arg(help_heading = "Config", long, help = format!("Write the default config to {}", DEFAULT_CONFIG_FILE_PATH.display()))]
    pub dump_default_config: bool,

    /// Specifies the config file to use
    #[arg(
        long,
        help_heading = "Config",
        value_name = "file.kdl",
        default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string_lossy().to_string()
    )]
    pub config_file: String,

    //
    // --- Debug ---
    //
    // These options are hidden from the user
    //
    /// Choose a minumum level at which to log
    #[arg(help_heading = "Debug", long, hide = true, default_value_t = log::LevelFilter::Error)]
    pub log_level: log::LevelFilter,
    /// Log to stdout instead of file
    #[arg(help_heading = "Debug", long, hide = true)]
    pub log_stdout: bool,
    /// Print the path of the log file
    #[arg(help_heading = "Debug", long, hide = true, default_value_t = DEFAULT_LOG_FILE_PATH.to_string_lossy().to_string())]
    pub log_file: String,
    /// Launch ferrishot in debug mode (F12)
    #[arg(help_heading = "Debug", long, hide = true)]
    pub debug: bool,
    /// Output the path to the log file
    #[arg(help_heading = "Debug", long, hide = true)]
    pub print_log_file_path: bool,

    /// Print markdown of the command line interface
    /// This looks nicer than just copy-pasting the command line output into a code block
    #[arg(long, hide = true)]
    pub markdown_help: bool,
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
