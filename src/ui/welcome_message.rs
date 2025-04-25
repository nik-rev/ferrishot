//! The welcome message contains tips on how to use ferrishot

use iced::{
    Background, Color, Element, Font, Length,
    alignment::Vertical,
    widget::{Space, column, row, text, text::Shaping},
};

use crate::{CONFIG, message::Message};

/// Width of the welcome message box
const WIDTH: u32 = 380;
/// Height of the welcome message box
const HEIGHT: u32 = 160;
/// Size of the font in the welcome message box
const FONT_SIZE: f32 = 13.0;

/// Renders the welcome message that the user sees when they first launch the program
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct WelcomeMessage {
    /// Width of the container of the welcome message
    pub image_width: u32,
    /// Height of the container of the welcome message
    pub image_height: u32,
}

impl WelcomeMessage {
    /// Render the welcome message
    pub fn view(self) -> Element<'static, Message> {
        let vertical_space = Space::with_height(self.image_height / 2 - HEIGHT / 2);
        let horizontal_space = Space::with_width(self.image_width / 2 - WIDTH / 2);

        let bold = Font {
            weight: iced::font::Weight::Bold,
            ..Font::default()
        };

        let keys = |key: &'static str, action: &'static str| {
            row![
                row![
                    Space::with_width(Length::Fill),
                    text(key)
                        .size(FONT_SIZE)
                        .font(bold)
                        .shaping(Shaping::Advanced)
                        .align_y(Vertical::Bottom)
                ]
                .width(100.0),
                Space::with_width(Length::Fixed(20.0)),
                text(action).size(FONT_SIZE).align_y(Vertical::Bottom),
            ]
        };

        let stuff = iced::widget::container(
            column![
                keys("Mouse", "Select screenshot area"),
                keys("Ctrl + S", "Save screenshot to a file"),
                keys("Enter", "Copy screenshot to clipboard"),
                keys("Right Click", "Snap closest corner to mouse"),
                keys("Shift + Mouse", "Slowly resize / move area"),
                keys("Esc", "Exit"),
            ]
            .spacing(8.0)
            .height(HEIGHT)
            .width(WIDTH)
            .padding(10.0),
        )
        .style(|_| iced::widget::container::Style {
            text_color: Some(CONFIG.theme.info_box_fg),
            background: Some(Background::Color(CONFIG.theme.info_box_bg)),
            border: iced::Border::default()
                .color(Color::WHITE)
                .rounded(6.0)
                .width(1.5),
            shadow: iced::Shadow::default(),
        });

        column![vertical_space, row![horizontal_space, stuff]].into()
    }
}
