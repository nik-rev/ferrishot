//! The welcome message contains tips on how to use ferrishot

use iced::{
    Background, Element, Font,
    Length::{self, Fill},
    alignment::Vertical,
    widget::{Space, column, row, text, text::Shaping},
};

use crate::message::Message;

/// Width of the welcome message box
const WIDTH: u32 = 380;
/// Height of the welcome message box
const HEIGHT: u32 = 160;
/// Size of the font in the welcome message box
const FONT_SIZE: f32 = 13.0;

/// Renders the welcome message that the user sees when they first launch the program
pub fn welcome_message(app: &super::App) -> Element<Message> {
    let image_width = app.image.width();
    let image_height = app.image.height();
    let vertical_space = Space::with_height(image_height / 2 - HEIGHT / 2);
    let horizontal_space = Space::with_width(image_width / 2 - WIDTH / 2);

    let bold = Font {
        weight: iced::font::Weight::Bold,
        ..Font::default()
    };

    let tip = |key: &'static str, action: &'static str| {
        row![
            row![
                Space::with_width(Fill),
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
            tip("Mouse", "Select screenshot area"),
            tip("Ctrl + S", "Save screenshot to a file"),
            tip("Enter", "Copy screenshot to clipboard"),
            tip("Right Click", "Snap closest corner to mouse"),
            tip("Shift + Mouse", "Slowly resize / move area"),
            tip("Esc", "Exit"),
        ]
        .spacing(8.0)
        .height(HEIGHT)
        .width(WIDTH)
        .padding(10.0),
    )
    .style(|_| iced::widget::container::Style {
        text_color: Some(app.config.theme.info_box_fg),
        background: Some(Background::Color(app.config.theme.info_box_bg)),
        border: iced::Border::default()
            .color(app.config.theme.info_box_border)
            .rounded(6.0)
            .width(1.5),
        shadow: iced::Shadow::default(),
    });

    column![vertical_space, row![horizontal_space, stuff]].into()
}
