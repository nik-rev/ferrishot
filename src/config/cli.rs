//! Parse the command line arguments passed to ferrishot
use std::time::Duration;
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;
use iced::Rectangle;

use crate::rect::RectangleExt;

/// Action to take when instantly accepting
#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum AcceptOnSelect {
    /// Copy the selected region to the clipboard
    Copy,
    /// Save the selected region as a file
    Save,
    /// Upload the selected region to the internet
    Upload,
}

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
#[expect(clippy::struct_excessive_bools, reason = "normal for CLIs")]
pub struct Cli {
    /// Instead of taking a screenshot of the desktop, open this image instead
    // Currently disabled because if the screenshot is not the same size as the desktop,
    // it will cause bugs as we consider 0,0 in the Canvas to be the
    #[arg(hide = true)]
    pub file: Option<PathBuf>,

    /// Screenshot region to select
    ///
    /// Format: <width>x<height>+<top-left-x>+<top-left-y>
    #[arg(long, value_parser = Rectangle::from_str, value_name = "WxH+X+Y")]
    pub region: Option<Rectangle>,

    /// Wait this long before launching
    #[arg(
        long,
        value_name = "MILLISECONDS",
        value_parser = |s: &str| s.parse().map(Duration::from_millis),
    )]
    pub delay: Option<Duration>,
    /// Instead of opening a file picker to save the screenshot, save it to this path instead
    #[arg(long, value_name = "PATH")]
    pub save_path: Option<PathBuf>,

    /// Accept capture as soon as a selection is made
    ///
    /// If holding `ctrl` while you are releasing the left mouse button on the
    /// first selection, the behaviour is cancelled
    #[arg(short('a'), long, value_name = "ACTION", verbatim_doc_comment)]
    pub accept_on_select: Option<AcceptOnSelect>,

    //
    // --- Config ---
    //
    // Currently these options are hidden. You *can* configure ferrishot,
    // but I'd like to expose this a bit later when I am sure this is the config I'd like to commit to
    //
    /// Write the default config file
    #[arg(long, hide = true, help = format!("Write the default config to {}", DEFAULT_CONFIG_FILE_PATH.display()))]
    pub dump_default_config: bool,
    /// Specifies the config file to use
    #[arg(
        long,
        hide = true,
        value_name = "file.kdl",
        default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string_lossy().to_string()
    )]
    pub config_file: String,

    //
    // --- Logging / Debugging ---
    //
    /// Choose a minumum level at which to log
    #[arg(group = "Logging", long, hide = true, default_value_t = log::LevelFilter::Error)]
    pub log_level: log::LevelFilter,
    /// Log to stdout instead of file
    #[arg(group = "Logging", long, hide = true)]
    pub log_stdout: bool,
    /// Print the path of the log file
    #[arg(group = "Logging", long, hide = true, default_value_t = DEFAULT_LOG_FILE_PATH.to_string_lossy().to_string())]
    pub log_file: String,
    /// Launch ferrishot in debug mode (F12)
    #[arg(long, hide = true)]
    pub debug: bool,
    /// Output the path to the log file
    #[arg(group = "Logging", long, hide = true)]
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
