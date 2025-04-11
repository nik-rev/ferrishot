//! Command line arguments to configure the program
use clap::Parser;

/// Configuration for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Config {
    /// The first selected rectangle will be instantly copied to the clipboard,
    /// and the app will exit.
    #[arg(long)]
    pub instant: bool,
}
