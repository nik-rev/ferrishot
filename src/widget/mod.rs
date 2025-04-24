//! Widgets with custom styles

use iced::Element;

mod app;
mod background_image;
mod errors;
mod icon;
mod icons;
mod letters;
pub mod selection;
mod size_indicator;
mod welcome_message;

pub use app::{App, SAVED_IMAGE};
pub use background_image::BackgroundImage;
pub use errors::Errors;
pub use icon::{icon, icon_tooltip};
pub use icons::Icons;
pub use letters::{Letters, PickCorner};
pub use size_indicator::SizeIndicator;
pub use welcome_message::WelcomeMessage;

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
