//! Widgets with custom styles
use iced::{Background, Border, Color, Element, Length, Shadow, Theme, color, widget};

use crate::{
    constants::{DROP_SHADOW, ICON_BUTTON_SIZE, ICON_SIZE},
    iced_aw::NumberInput,
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
        .style(|theme: &Theme, _| crate::iced_aw::number_input::Style {
            button_background: Some(iced::Background::Color(theme.palette().background)),
            icon_color: theme.palette().primary,
        })
        .input_style(|theme, _| widget::text_input::Style {
            background: Background::Color(theme.extended_palette().background.weak.color),
            border: Border::default(),
            icon: theme.palette().primary,
            placeholder: theme.palette().text,
            value: theme.extended_palette().background.weak.text,
            selection: theme.extended_palette().primary.weak.color,
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
        .style(|theme| widget::container::Style {
            text_color: Some(theme.palette().background),
            background: Some(Background::Color(theme.palette().text)),
            border: Border::default(),
            shadow: Shadow::default(),
        })
        .gap(10.0)
}

/// Styled icon as a button
pub fn icon<'a, Message>(icon: crate::icons::Icon) -> widget::Button<'a, Message> {
    widget::button(
        widget::Svg::new(icon.svg())
            .style(|theme: &Theme, _| widget::svg::Style {
                color: Some(theme.extended_palette().primary.base.text),
            })
            .width(Length::Fixed(ICON_SIZE))
            .height(Length::Fixed(ICON_SIZE)),
    )
    .width(Length::Fixed(ICON_BUTTON_SIZE))
    .height(Length::Fixed(ICON_BUTTON_SIZE))
    .style(move |theme: &Theme, _| {
        let mut style = widget::button::Style::default().with_background(theme.palette().primary);
        style.shadow = DROP_SHADOW;
        style.border = iced::Border::default()
            .rounded(iced::border::Radius::new(iced::Pixels::from(f32::INFINITY)));
        style
    })
}
