//! Widgets with custom styles

use iced::Element;

mod app;
mod background_image;
mod icon;
mod size_indicator;

pub use background_image::BackgroundImage;
pub use icon::{icon, icon_tooltip};
pub use size_indicator::size_indicator;

#[easy_ext::ext(Explainer)]
/// An extension trait to show a red border around an element and all children
pub impl<'a, M: 'a, E> E
where
    E: Into<Element<'a, M>>,
{
    #[expect(
        clippy::allow_attributes,
        reason = "so we dont have to switch between expect/allow"
    )]
    #[allow(dead_code, reason = "useful to exist for debugging")]
    /// Shows red border around an element and all of its children
    fn explain(self) -> Element<'a, M> {
        self.into().explain(iced::Color::from_rgb8(255, 0, 0))
    }
}
