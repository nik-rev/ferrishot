//! Shows useful information when pressing F12

use iced::{Background, Element};

use crate::message::Message;

use super::App;

/// Debug overlay shows useful information when pressing F12
pub struct DebugOverlay<'a> {
    /// Overlay for the application
    pub app: &'a App,
}

impl DebugOverlay<'_> {
    /// Render the debug overlay
    pub fn view(&self) -> Element<'static, Message> {
        iced::widget::container(
            iced::widget::column![iced::widget::text!(
                "image width: {}px | image height: {}px",
                self.app.image.width(),
                self.app.image.height()
            )]
            .push_maybe(self.app.selection.map(|sel| {
                iced::widget::text!(
                    "selection top-left corner: ({}, {})",
                    sel.rect.x,
                    sel.rect.y
                )
            })),
        )
        .style(|_| iced::widget::container::Style {
            text_color: Some(iced::Color::WHITE),
            background: Some(Background::Color(iced::Color::BLACK.scale_alpha(0.8))),
            ..Default::default()
        })
        .into()
    }
}
