//! The ferrishot app

/// Additional extension methods for Result
#[easy_ext::ext(ResultExt)]
pub impl<T, E: std::fmt::Debug + std::fmt::Display> Result<T, E> {
    /// Like `Result::expect`, but also logs the failure
    fn log_expect(self, message: &str) -> T {
        self.inspect_err(|err| log::error!("{message}: {err}"))
            .expect(message)
    }
}

mod app;
mod background_image;
mod clipboard;
mod config;
mod corners;
mod logging;
mod message;
mod mouse;
mod rectangle;
mod screenshot;
mod selection;

/// When there is an error, display it for this amount of time
const ERROR_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
/// Radius of the 4 corners of the selection
const CORNER_RADIUS: f32 = 6.0;
/// Color of the selection stroke and corners
const SELECTION_COLOR: iced::Color = iced::Color::WHITE;
/// The area around each side which allows that side to be hovered over and
/// resized
const INTERACTION_AREA: f32 = 30.0;
/// The size of the border of the square
const STROKE_SIZE: f32 = 2.0;
/// The color of the background for non-selected regions
const SHADE_COLOR: iced::Color = iced::color!(0x00_00_00, 0.15);

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
pub use logging::initialize_logging;
