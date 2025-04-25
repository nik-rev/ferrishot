//! Renders a tiny numeric input which shows a dimension of the rect and allow resizing it

use iced::{
    Background, Element, Length, Rectangle,
    widget::{self, Space, column, row, text::Shaping},
};

use crate::{CONFIG, message::Message, rect::RectangleExt as _, ui::selection::SelectionIsSome};

/// Shows the width and height of the image
pub struct SizeIndicator {
    /// Height of the image
    pub image_height: u32,
    /// Width of the image
    pub image_width: u32,
    /// Selection area
    pub selection_rect: Rectangle,
    /// A key to guarantee that `Selection.is_some()`
    pub sel_is_some: SelectionIsSome,
}

/// Renders the indicator for a single dimension (e.g. width or height)
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

impl SizeIndicator {
    /// Renders a tiny numeric input which shows a dimension of the rect and allow resizing it
    pub fn view(self) -> Element<'static, Message> {
        const SPACING: f32 = 12.0;
        const ESTIMATED_INDICATOR_WIDTH: u32 = 120;
        const ESTIMATED_INDICATOR_HEIGHT: u32 = 26;

        let x_offset = (self.selection_rect.bottom_right().x + SPACING)
            .min((self.image_width - ESTIMATED_INDICATOR_WIDTH) as f32);
        let y_offset = (self.selection_rect.bottom_right().y + SPACING)
            .min((self.image_height - ESTIMATED_INDICATOR_HEIGHT) as f32);

        let horizontal_space = Space::with_width(x_offset);
        let vertical_space = Space::with_height(y_offset);

        let width = dimension_indicator(self.selection_rect.width as u32, move |new_width| {
            Message::ResizeHorizontally {
                new_width,
                sel_is_some: self.sel_is_some,
            }
        });
        let height = dimension_indicator(self.selection_rect.height as u32, move |new_height| {
            Message::ResizeVertically {
                new_height,
                sel_is_some: self.sel_is_some,
            }
        });
        let x = iced::widget::text("✕ ")
            .color(CONFIG.theme.size_indicator_fg)
            .shaping(Shaping::Advanced);
        let space = iced::widget::text(" ");
        let c =
            widget::container(row![space, width, x, height]).style(|_| widget::container::Style {
                text_color: None,
                background: Some(Background::Color(CONFIG.theme.size_indicator_bg)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            });

        column![vertical_space, row![horizontal_space, c]].into()
    }
}
