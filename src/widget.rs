//! Widgets with custom styles
use iced::{Background, Border, Element, Length, Shadow, Theme, widget};

use crate::constants::{DROP_SHADOW, ICON_BUTTON_SIZE, ICON_SIZE};

/// Create a new tooltip
pub fn tooltip<'a, Message>(
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
