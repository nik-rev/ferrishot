//! Parse the command line arguments passed to ferrishot
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Cli {
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
    // --- Logging ---
    //
    /// Choose a minumum level at which to log
    #[arg(group = "Logging", long, hide = true, default_value_t = log::LevelFilter::Error)]
    pub log_level: log::LevelFilter,
    /// Log to stdout instead of file
    #[arg(group = "Logging", long, hide = true)]
    pub log_stdout: bool,
    /// Print the path of the log file
    #[arg(group = "Logging",long, hide = true, default_value_t = DEFAULT_LOG_FILE_PATH.to_string_lossy().to_string())]
    pub log_file: String,
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

/// Command line arguments passed to ferrishot
///
/// It is a static because it is needed by the `CONFIG` static, in order to
/// read config from the correct place
pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);
