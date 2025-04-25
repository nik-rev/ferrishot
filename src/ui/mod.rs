//! Widgets with custom styles

use iced::Element;

mod app;
mod background_image;
mod debug_overlay;
mod errors;
pub mod selection;
mod selection_icons;
mod welcome_message;

use background_image::BackgroundImage;
use debug_overlay::DebugOverlay;
use errors::Errors;

pub mod size_indicator;
use size_indicator::SizeIndicator;

pub mod image_uploaded;
use image_uploaded::ImageUploaded;

pub mod letters;
use letters::{Letters, PickCorner};

use selection_icons::SelectionIcons;
use welcome_message::WelcomeMessage;

pub use app::{App, SAVED_IMAGE};

/// An extension trait to show a red border around an element and all children
#[easy_ext::ext(Explainer)]
pub impl<'a, M: 'a, E> E
where
    E: Into<Element<'a, M>>,
{
    /// Shows red border around an element and all of its children
    #[expect(
        clippy::allow_attributes,
        reason = "so we dont have to switch between expect/allow"
    )]
    #[allow(dead_code, reason = "useful to exist for debugging")]
    fn explain(self) -> Element<'a, M> {
        self.into().explain(iced::Color::from_rgb8(255, 0, 0))
    }
}
