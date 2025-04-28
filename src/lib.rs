//! The ferrishot app

/// A single client for HTTP requests
static CLIENT: std::sync::LazyLock<reqwest::Client> =
    std::sync::LazyLock::new(reqwest::Client::new);

mod clipboard;
mod config;
mod icons;
mod image_upload;
pub mod logging;
mod message;
mod rect;
mod screenshot;
mod ui;

use message::Message;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use config::{Cli, Config, DEFAULT_KDL_CONFIG_STR, DEFAULT_LOG_FILE_PATH};
pub use ui::{App, SAVED_IMAGE};
