//! The ferrishot app

// TODO: once iced v14 releases,
// and this crate updates to use this version,
// we don't need to vendor it anymore
mod iced_aw;

mod theme;
use iced_aw::style;
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
mod stdx;
mod widgets;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
pub use config::CONFIG;
pub use logging::initialize_logging;
pub use stdx::foreground_for;
