//! Show a popup when the image has already been uploaded
//! Popup contains:
//! - QR Code
//! - Image thumbnail
//! - Copy URL to clipboard

use iced::{
    Background, Element, Length,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, qr_code, row, svg, text,
        vertical_space,
    },
};

/// Data of the uploaded image
#[derive(Clone, Debug)]
pub struct ImageUploadedData {
    /// link to the uploaded image
    pub url: String,
    /// the uploaded image
    pub uploaded_image: iced::widget::image::Handle,
    /// The height of the image
    pub height: u32,
    /// The width of the image
    pub width: u32,
    /// File size in bytes
    pub file_size: u64,
}

/// Message for the image uploaded
#[derive(Clone, Debug)]
pub enum Message {
    /// Close image uploaded popup
    CloseImageUploadedPopup,
    /// Click "close" on the image upload menu
    ExitImageUploadMenu,
    /// The image was uploaded to the internet
    ImageUploaded(ImageUploadedData),
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
            Self::ImageUploaded(data) => {
                app.uploaded_url = Some((
                    iced::widget::qr_code::Data::new(data.url.clone())
                        .expect("URL to be valid QR Code data"),
                    data,
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
    /// Data of the uploaded image
    pub data: &'app ImageUploadedData,
}

impl<'app> ImageUploaded<'app> {
    /// Render the QR Code
    pub fn view(&self) -> Element<'app, crate::Message> {
        // let close_button = button(icon!(Close).style(|_, _| svg::Style {
        //     color: Some(iced::Color::WHITE),
        // }))
        // .width(Length::Fixed(32.0))
        // .height(Length::Fixed(32.0))
        // .style(|_, _| button::Style {
        //     background: Some(Background::Color(Color::TRANSPARENT)),
        //     text_color: Color::TRANSPARENT,
        //     ..Default::default()
        // })
        // .on_press(crate::Message::ImageUploaded(
        //     Message::CloseImageUploadedPopup,
        // ));

        container(
            container(
                column![
                    //
                    // Heading
                    //
                    container(text("Image Uploaded").size(30.0)).center_x(Length::Fill),
                    //
                    // Divider
                    //
                    container(horizontal_rule(2)).height(10.0),
                    //
                    // URL Text + Copy Button + QR Code
                    //
                    container(
                        column![
                            //
                            // URL Text + Copy Button
                            //
                            container(row![
                                //
                                // URL Text
                                //
                                container(text(self.data.url.clone()).color(iced::Color::WHITE))
                                    .center_y(Length::Fill),
                                //
                                // Copy to clipboard button
                                //
                                container(
                                    button(
                                        icon!(Clipboard)
                                            .width(Length::Fixed(25.0))
                                            .height(Length::Fixed(25.0))
                                            .style(|_, _| {
                                                svg::Style {
                                                    color: Some(iced::Color::WHITE),
                                                }
                                            })
                                    )
                                    .on_press(crate::Message::ImageUploaded(Message::CopyLink(
                                        self.data.url.to_string()
                                    )))
                                    .style(|_, _| {
                                        iced::widget::button::Style {
                                            background: Some(Background::Color(
                                                iced::Color::TRANSPARENT,
                                            )),
                                            ..Default::default()
                                        }
                                    })
                                )
                                .center_y(Length::Fill)
                            ])
                            .style(|_| container::Style {
                                text_color: Some(iced::Color::WHITE),
                                background: Some(Background::Color(iced::color!(0x0f_0f_0f))),
                                ..Default::default()
                            })
                            .center_y(Length::Fixed(32.0))
                            .center_x(Length::Fill),
                            //
                            // QR Code
                            //
                            container(qr_code(self.qr_code_data).total_size(250.0))
                                .center_x(Length::Fill),
                        ]
                        .spacing(30.0)
                    )
                    .center(Length::Fill)
                    .height(Length::Fixed(300.0)),
                    //
                    // Heading
                    //
                    container(text("Preview").size(30.0)).center_x(Length::Fill),
                    //
                    // Metadata
                    //
                    container(column![
                        text!("size: {w} ✕ {h}", w = self.data.width, h = self.data.height)
                            .shaping(text::Shaping::Advanced),
                        text!(
                            "filesize: {}",
                            human_bytes::human_bytes(self.data.file_size as f64)
                        )
                    ])
                    .center_x(Length::Fill),
                    //
                    // Image
                    //
                    iced::widget::image(self.data.uploaded_image.clone()).width(Length::Fill)
                ]
                .spacing(30.0),
            )
            .width(Length::Fixed(700.0))
            .height(Length::Fixed(1100.0))
            .style(|_| container::Style {
                text_color: Some(iced::Color::WHITE),
                background: Some(Background::Color(iced::Color::BLACK.scale_alpha(0.9))),
                ..Default::default()
            })
            .padding(30.0),
        )
        .center(Length::Fill)
        .into()
    }
}
