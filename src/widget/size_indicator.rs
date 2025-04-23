//! Renders a tiny numeric input which shows a dimension of the rect and allow resizing it

use iced::{
    Background, Element, Length, Rectangle,
    widget::{self, Space, column, row, text::Shaping},
};

use crate::{
    CONFIG, message::Message, rectangle::RectangleExt as _,
    selection::selection_lock::SelectionIsSome,
};

/// Renders a tiny numeric input which shows a dimension of the rect and allow resizing it
pub fn size_indicator<'a>(
    image_height: u32,
    image_width: u32,
    rect: Rectangle,
    sel_is_some: SelectionIsSome,
) -> Element<'a, Message> {
    fn dimension_indicator<'a>(
        value: u32,
        on_change: impl Fn(u32) -> Message + 'a,
    ) -> widget::TextInput<'a, Message> {
        let content = value.to_string();
        let input = iced::widget::text_input(Default::default(), content.as_str())
            // HACK: iced does not provide a way to mimic `width: min-content` from CSS
            // so we have to "guesstimate" the width that each character will be
            // `Length::Shrink` makes `width = 0` for some reason
            .width(Length::Fixed((12 * content.len()) as f32))
            .on_input(move |s| {
                // if we get "" it means user e.g. just deleted everything
                if s.is_empty() {
                    on_change(0)
                } else {
                    s.parse::<u32>().ok().map_or(Message::NoOp, &on_change)
                }
            })
            .style(|_, _| widget::text_input::Style {
                value: CONFIG.theme.size_indicator_fg,
                selection: CONFIG.theme.text_selection,
                // --- none
                background: Background::Color(iced::Color::TRANSPARENT),
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                icon: iced::Color::TRANSPARENT,
                placeholder: iced::Color::TRANSPARENT,
            })
            .padding(0.0);

        input
    }

    const SPACING: f32 = 12.0;
    const ESTIMATED_INDICATOR_WIDTH: u32 = 120;
    const ESTIMATED_INDICATOR_HEIGHT: u32 = 26;

    let x_offset =
        (rect.bottom_right().x + SPACING).min((image_width - ESTIMATED_INDICATOR_WIDTH) as f32);
    let y_offset =
        (rect.bottom_right().y + SPACING).min((image_height - ESTIMATED_INDICATOR_HEIGHT) as f32);

    let horizontal_space = Space::with_width(x_offset);
    let vertical_space = Space::with_height(y_offset);

    let width = dimension_indicator(rect.width as u32, move |new_width| {
        Message::ResizeHorizontally {
            new_width,
            sel_is_some,
        }
    });
    let height = dimension_indicator(rect.height as u32, move |new_height| {
        Message::ResizeVertically {
            new_height,
            sel_is_some,
        }
    });
    let x = iced::widget::text("✕ ")
        .color(CONFIG.theme.size_indicator_fg)
        .shaping(Shaping::Advanced);
    let space = iced::widget::text(" ");
    let c = widget::container(row![space, width, x, height]).style(|_| widget::container::Style {
        text_color: None,
        background: Some(Background::Color(CONFIG.theme.size_indicator_bg)),
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
    });

    column![vertical_space, row![horizontal_space, c]].into()
}
