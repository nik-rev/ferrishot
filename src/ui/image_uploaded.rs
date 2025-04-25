//! Show a popup when the image has already been uploaded
//! Popup contains:
//! - QR Code
//! - Image thumbnail
//! - Copy URL to clipboard

use iced::{
    Background, Element, Length,
    widget::{
        button, column, container, horizontal_space, image, image::Handle, qr_code, row, svg, text,
        vertical_space,
    },
};

/// Message for the image uploaded
#[derive(Clone, Debug)]
pub enum Message {
    /// Close image uploaded popup
    CloseImageUploadedPopup,
    /// Click "close" on the image upload menu
    ExitImageUploadMenu,
    /// The image was uploaded to the internet
    ImageUploaded {
        /// link to the uploaded image
        url: String,
        /// the uploaded image
        uploaded_image: iced::widget::image::Handle,
    },
    /// Copy link of image to clipboard
    CopyLink(String),
}

impl crate::message::Handler for Message {
    /// Handle the message
    fn handle(self, app: &mut super::App) {
        match self {
            Self::CopyLink(url) => {
                if let Err(err) = crate::clipboard::set_text(&url) {
                    app.errors.push(err.to_string());
                }
            }
            Self::CloseImageUploadedPopup => {
                app.uploaded_url = None;
            }
            Self::ExitImageUploadMenu => app.uploaded_url = None,
            Self::ImageUploaded {
                url,
                uploaded_image,
            } => {
                app.uploaded_url = Some((
                    iced::widget::qr_code::Data::new(&url).unwrap(),
                    uploaded_image,
                    url,
                ));
            }
        }
    }
}

use crate::icon;

/// Data for the uploaded image
pub struct ImageUploaded<'app> {
    /// Data for the URL to the uploaded image
    pub qr_code_data: &'app qr_code::Data,
    /// Link to the URL to the uploaded image
    pub url: &'app str,
    /// The uploaded image
    pub image_handle: &'app Handle,
}

impl<'app> ImageUploaded<'app> {
    /// Render the QR Code
    pub fn view(&self) -> Element<'app, crate::Message> {
        container(
            container(column![
                row![
                    container(row![
                        text(self.url).color(iced::Color::WHITE),
                        button(icon!(Clipboard).style(|_, _| svg::Style {
                            color: Some(iced::Color::WHITE)
                        }))
                        .on_press(crate::Message::ImageUploaded(Message::CopyLink(
                            self.url.to_string()
                        )))
                        .style(|_, _| {
                            iced::widget::button::Style {
                                background: Some(Background::Color(iced::Color::TRANSPARENT)),
                                ..Default::default()
                            }
                        })
                    ])
                    .style(|_| {
                        container::Style {
                            text_color: Some(iced::Color::WHITE),
                            background: Some(Background::Color(iced::color!(0x0f_0f_0f))),
                            ..Default::default()
                        }
                    }),
                    horizontal_space().width(Length::Fill),
                    button(icon!(Close).style(|_, _| svg::Style {
                        color: Some(iced::Color::WHITE)
                    }))
                    .on_press(crate::Message::ImageUploaded(
                        Message::CloseImageUploadedPopup
                    ))
                ],
                vertical_space().height(Length::Fill),
                container(row![
                    container(qr_code(self.qr_code_data).total_size(200.0))
                        .height(300.0)
                        .center_y(Length::Fill),
                    horizontal_space().width(30.0),
                    horizontal_space().width(Length::Fill),
                    image(self.image_handle).height(300.0),
                ]),
            ])
            .padding(40.0)
            .width(Length::Fixed(1200.0))
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
