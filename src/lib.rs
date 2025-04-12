//! The ferrishot utility functions

mod app;
mod background_image;
mod clipboard;
mod config;
mod corners;
mod message;
mod mouse;
mod rectangle;
mod screenshot;
mod selection;

/// Radius of the 4 corners of the selection
pub const CORNER_RADIUS: f32 = 6.0;
/// Color of the selection stroke and corners
pub const SELECTION_COLOR: iced::Color = iced::Color::WHITE;
/// The area around each side which allows that side to be hovered over and
/// resized
pub const INTERACTION_AREA: f32 = 30.0;
/// The size of the border of the square
pub const STROKE_SIZE: f32 = 2.0;
/// The color of the background for non-selected regions
pub const SHADE_COLOR: iced::Color = iced::color!(0x00_00_00, 0.15);

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
