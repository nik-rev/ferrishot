//! Parse the command line arguments passed to ferrishot
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Cli {
    /// Write the default config file
    #[arg(long, help = format!("Write the default config to {}", DEFAULT_CONFIG_FILE_PATH.display()))]
    pub dump_default_config: bool,
    /// Specifies the config file to use
    #[arg(
        long,
        value_name = "file.kdl",
        default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string_lossy().to_string()
    )]
    pub config_file: String,
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

/// Command line arguments to this program
///
/// It is a static because it is needed by the `CONFIG` static, in order to
/// read config from the correct place
pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);
