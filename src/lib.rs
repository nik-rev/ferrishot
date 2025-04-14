//! The ferrishot app

mod app;
mod background_image;
mod clipboard;
mod config;
mod constants;
mod corners;
mod icons;
mod logging;
mod message;
mod mouse;
mod rectangle;
mod screenshot;
mod selection;
mod widget;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
pub use logging::initialize_logging;
