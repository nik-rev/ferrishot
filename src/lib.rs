//! The ferrishot app

mod clipboard;
mod config;
mod icons;
mod image;
mod last_region;
mod message;
mod rect;
mod ui;

use config::Theme;
use message::Message;

pub mod logging;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use config::{Cli, Config, DEFAULT_KDL_CONFIG_STR, DEFAULT_LOG_FILE_PATH};
pub use image::action::SAVED_IMAGE;
pub use image::get_image;
pub use last_region::LastRegion;
pub use ui::App;
