//! The ferrishot app

use iced::Element;

/// A single client for HTTP requests
static CLIENT: std::sync::LazyLock<reqwest::Client> =
    std::sync::LazyLock::new(reqwest::Client::new);

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
mod canvas;
mod clipboard;
mod config;
mod corners;
mod icons;
mod image_upload;
mod message;
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
pub use config::Config;
