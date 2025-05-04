//! Popups are overlaid on top of the screen.
//! They block any inputs

use iced::widget as w;
pub mod keybindings_cheatsheet;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length::Fill;
pub use keybindings_cheatsheet::KeybindingsCheatsheet;

pub mod image_uploaded;
pub use image_uploaded::ImageUploaded;

pub mod letters;
pub use letters::{Letters, PickCorner};

/// Popup are overlaid on top and they block any events. allowing only Escape to close
/// the popup.
#[derive(Debug, strum::EnumTryAs)]
pub enum Popup {
    /// Letters allow picking a one of 10,000+ regions on the screen in 4 keystrokes
    Letters(letters::State),
    /// An image has been uploaded to the internet
    ImageUploaded(image_uploaded::State),
    /// Shows available commands
    KeyCheatsheet,
}

/// Make element full screen
fn popup<'a>(
    size: iced::Size,
    contents: impl Into<Element<'a, crate::Message>>,
) -> Element<'a, crate::Message> {
    w::container(w::stack![
        contents.into(),
        //
        // Close Button 'x' in the top right corner
        //
        w::column![
            w::vertical_space().height(10.0),
            w::row![
                w::horizontal_space().width(Fill),
                w::button(
                    crate::icon!(Close)
                        .style(|_, _| w::svg::Style {
                            color: Some(Color::WHITE)
                        })
                        .width(24.0)
                        .height(24.0)
                )
                .on_press(crate::Message::ClosePopup)
                .style(|_, _| w::button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    ..Default::default()
                }),
                w::horizontal_space().width(10.0)
            ]
            .height(size.height)
            .width(size.width)
        ]
    ])
    .center(Fill)
    .into()
}
