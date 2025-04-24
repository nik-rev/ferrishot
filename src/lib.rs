//! The ferrishot app

/// A single client for HTTP requests
static CLIENT: std::sync::LazyLock<reqwest::Client> =
    std::sync::LazyLock::new(reqwest::Client::new);

mod clipboard;
mod config;
mod corners;
mod icons;
mod image_upload;
mod message;
mod rectangle;
mod screenshot;
mod widget;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use config::{CLI, CONFIG, Config, DEFAULT_KDL_CONFIG_STR};
pub use widget::{App, SAVED_IMAGE};
