//! Command line arguments to configure the program
use clap::Parser;

/// Configuration for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Config {
    /// The first selection will be copied to the clipboard as soon as the left mouse button is released
    #[arg(long)]
    pub instant: bool,
}
