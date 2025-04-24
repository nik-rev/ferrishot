//! Data for the uploaded image

use iced::{
    Background, Element, Length,
    widget::{container, qr_code},
};

use crate::message::Message;

/// Data for the uploaded image
pub struct ImageUploaded<'app> {
    /// Data for the QR Code: the URL to the uploaded image
    pub qr_code_data: &'app qr_code::Data,
}

impl<'app> ImageUploaded<'app> {
    /// Render the QR Code
    pub fn view(&self) -> Element<'app, Message> {
        container(
            container(qr_code(self.qr_code_data))
                .width(Length::Fixed(650.0))
                .height(Length::Fixed(650.0))
                .style(|_| container::Style {
                    text_color: Some(iced::Color::WHITE),
                    background: Some(Background::Color(iced::Color::BLACK)),
                    ..Default::default()
                }),
        )
        .center(Length::Fill)
        .into()
    }
}
