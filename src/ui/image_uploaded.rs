//! Show a popup when the image has already been uploaded
//!
//! Popup contains:
//!
//! - QR Code
//! - Copy URL to clipboard
//! - Image metadata
//! - Image preview

use std::{thread, time::Duration};

use iced::{
    Background, Color, Element,
    Length::{self, Fill},
    Task,
    widget::{button, column, container, horizontal_rule, qr_code, row, stack, svg, text, tooltip},
};

use crate::icon;

use super::selection_icons::icon_tooltip;

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
    /// Some time has passed after the link was copied
    CopyLinkTimeout,
}

impl crate::message::Handler for Message {
    fn handle(self, app: &mut super::App) -> Option<Task<crate::Message>> {
        match self {
            Self::CopyLinkTimeout => app.has_copied_uploaded_image_link = false,
            Self::CopyLink(url) => {
                if let Err(err) = crate::clipboard::set_text(&url) {
                    app.errors.push(err.to_string());
                } else {
                    app.has_copied_uploaded_image_link = true;
                    return Some(Task::future(async move {
                        thread::sleep(Duration::from_secs(3));
                        crate::Message::ImageUploaded(Self::CopyLinkTimeout)
                    }));
                }
            }
            Self::CloseImageUploadedPopup => {
                app.uploaded_url = None;
            }
            Self::ExitImageUploadMenu => app.uploaded_url = None,
            Self::ImageUploaded(data) => match iced::widget::qr_code::Data::new(data.url.clone()) {
                Ok(qr_code) => {
                    app.uploaded_url = Some((qr_code, data));
                }
                Err(err) => {
                    app.errors.push(format!("Failed to get QR Code: {err}"));
                }
            },
        }

        None
    }
}

/// Data for the uploaded image
pub struct ImageUploaded<'app> {
    /// Data for the URL to the uploaded image
    pub qr_code_data: &'app qr_code::Data,
    /// When the URL Was copied
    pub url_copied: bool,
    /// Data of the uploaded image
    pub data: &'app ImageUploadedData,
}

impl<'app> ImageUploaded<'app> {
    /// Render the QR Code
    pub fn view(&self) -> Element<'app, crate::Message> {
        container(
            container(stack![
                column![
                    //
                    // Heading
                    //
                    container(text("Image Uploaded").size(30.0)).center_x(Fill),
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
                                    .center_y(Fill),
                                //
                                // Copy to clipboard button
                                //
                                {
                                    let (clipboard_icon, clipboard_icon_color, label) = if self
                                        .url_copied
                                    {
                                        (icon!(Check), iced::color!(0x_00_ff_00), "Copied!")
                                    } else {
                                        (icon!(Clipboard), iced::color!(0x_ff_ff_ff), "Copy Link")
                                    };

                                    container(icon_tooltip(
                                        button(
                                            clipboard_icon
                                                .style(move |_, _| svg::Style {
                                                    color: Some(clipboard_icon_color),
                                                })
                                                .width(Length::Fixed(25.0))
                                                .height(Length::Fixed(25.0)),
                                        )
                                        .on_press(crate::Message::ImageUploaded(Message::CopyLink(
                                            self.data.url.to_string(),
                                        )))
                                        .style(|_, _| {
                                            iced::widget::button::Style {
                                                background: Some(Background::Color(
                                                    iced::Color::TRANSPARENT,
                                                )),
                                                ..Default::default()
                                            }
                                        }),
                                        text(label),
                                        tooltip::Position::Top,
                                    ))
                                    .center_y(Fill)
                                }
                            ])
                            .style(|_| container::Style {
                                text_color: Some(iced::Color::WHITE),
                                ..Default::default()
                            })
                            .center_y(Length::Fixed(32.0))
                            .center_x(Fill),
                            //
                            // QR Code
                            //
                            container(qr_code(self.qr_code_data).total_size(250.0)).center_x(Fill),
                        ]
                        .spacing(30.0)
                    )
                    .center(Fill)
                    .height(Length::Fixed(300.0)),
                    //
                    // --- Preview ---
                    //
                    container(
                        row![
                            container(horizontal_rule(2)).center_y(Fill),
                            container(text("Preview").size(30.0)).center_y(Fill),
                            container(horizontal_rule(2)).center_y(Fill)
                        ]
                        .spacing(20.0)
                    )
                    .center_x(Fill),
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
                    .center_x(Fill),
                    //
                    // Image
                    //
                    iced::widget::image(self.data.uploaded_image.clone()).width(Fill)
                ]
                .spacing(30.0),
                //
                // Close button
                //
                container(icon_tooltip(
                    button(
                        icon!(Close)
                            .width(30.0)
                            .height(30.0)
                            .style(|_, _| svg::Style {
                                color: Some(Color::WHITE)
                            })
                    )
                    .style(|_, _| button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        ..Default::default()
                    })
                    .on_press(crate::Message::ImageUploaded(
                        Message::CloseImageUploadedPopup
                    )),
                    text("Close"),
                    tooltip::Position::Right
                ))
                .align_top(Fill)
                .align_right(Fill),
            ])
            .width(Length::Fixed(700.0))
            .height(Length::Fixed(1100.0))
            .style(|_| container::Style {
                text_color: Some(iced::Color::WHITE),
                background: Some(Background::Color(iced::Color::BLACK.scale_alpha(0.9))),
                ..Default::default()
            })
            .padding(30.0),
        )
        .center(Fill)
        .into()
    }
}
