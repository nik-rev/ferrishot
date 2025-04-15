//! The ferrishot app

use iced::Element;

/// An extension trait to show a red border around an element and all children
pub trait Explainer<'a, M> {
    /// Shows red border around an element and all of its children
    fn explain(self) -> Element<'a, M>;
}

impl<'a, M: 'a, E> Explainer<'a, M> for E
where
    E: Into<Element<'a, M>>,
{
    fn explain(self) -> Element<'a, M> {
        self.into().explain(iced::Color::from_rgb8(255, 0, 0))
    }
}

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
mod theme;
mod widgets;

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
pub use config::CONFIG;
pub use logging::initialize_logging;
