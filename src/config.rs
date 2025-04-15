//! Command line arguments to configure the program
use std::sync::LazyLock;

use clap::Parser;

/// Configuration of the app
pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

/// Configuration for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Config {
    /// The first selection will be copied to the clipboard as soon as the left mouse button is released
    #[arg(long)]
    pub instant: bool,
}
