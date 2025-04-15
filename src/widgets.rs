//! Widgets with custom styles
use iced::{Background, Border, Element, Length, Shadow, widget};

use crate::{
    constants::{ICON_BUTTON_SIZE, ICON_SIZE},
    iced_aw::NumberInput,
    theme::THEME,
};

/// Renders a tiny numeric input which shows a dimension of the rect and allow resizing it
#[expect(
    clippy::cast_precision_loss,
    reason = "as we do not need to be precise"
)]
pub fn dimension_indicator<'a, Message, F>(
    value: u32,
    bounds: impl std::ops::RangeBounds<u32>,
    on_change: F,
) -> NumberInput<'a, u32, Message>
where
    F: Fn(u32) -> Message + Clone + 'static,
    Message: Clone + 'a,
{
    crate::iced_aw::NumberInput::new(&value, bounds, on_change)
        .style(|_, _| crate::iced_aw::number_input::Style {
            button_background: Some(Background::Color(THEME.bg)),
            icon_color: THEME.accent,
        })
        .input_style(|_, _| widget::text_input::Style {
            background: Background::Color(THEME.size_indicator_bg),
            border: Border::default(),
            icon: THEME.accent,
            placeholder: THEME.fg,
            value: THEME.size_indicator_fg,
            selection: THEME.text_selection_bg,
        })
        .ignore_buttons(true)
        .width(15.0 * value.to_string().len() as f32)
}

/// Create a tooltip for an icon
pub fn icon_tooltip<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
    position: widget::tooltip::Position,
) -> widget::Tooltip<'a, Message> {
    widget::Tooltip::new(content, tooltip, position)
        .style(|_| widget::container::Style {
            text_color: Some(THEME.fg),
            background: Some(Background::Color(THEME.bg)),
            border: Border::default(),
            shadow: Shadow::default(),
        })
        .gap(10.0)
}

/// Styled icon as a button
pub fn icon<'a, Message>(icon: crate::icons::Icon) -> widget::Button<'a, Message> {
    widget::button(
        widget::Svg::new(icon.svg())
            .style(|_, _| widget::svg::Style {
                color: Some(THEME.fg_on_accent_bg),
            })
            .width(Length::Fixed(ICON_SIZE))
            .height(Length::Fixed(ICON_SIZE)),
    )
    .width(Length::Fixed(ICON_BUTTON_SIZE))
    .height(Length::Fixed(ICON_BUTTON_SIZE))
    .style(move |_, _| {
        let mut style = widget::button::Style::default().with_background(THEME.accent);
        style.shadow = Shadow {
            color: THEME.drop_shadow,
            blur_radius: 3.0,
            offset: iced::Vector { x: 0.0, y: 0.0 },
        };
        style.border = iced::Border::default()
            .rounded(iced::border::Radius::new(iced::Pixels::from(f32::INFINITY)));
        style
    })
}
