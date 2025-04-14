//! Widgets with custom styles
use iced::{Background, Border, Color, Element, Length, Shadow, widget};

use crate::constants::{DROP_SHADOW, ICON_BACKGROUND, ICON_BUTTON_SIZE, ICON_COLOR, ICON_SIZE};

/// Create a new tooltip
pub fn tooltip<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
    position: widget::tooltip::Position,
) -> widget::Tooltip<'a, Message> {
    widget::Tooltip::new(content, tooltip, position)
        .style(|_| widget::container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(iced::color!(0x00_00_00, 0.3))),
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
                color: Some(ICON_COLOR),
            })
            .width(Length::Fixed(ICON_SIZE))
            .height(Length::Fixed(ICON_SIZE)),
    )
    .width(Length::Fixed(ICON_BUTTON_SIZE))
    .height(Length::Fixed(ICON_BUTTON_SIZE))
    .style(|_, _| {
        let mut style = widget::button::Style::default().with_background(ICON_BACKGROUND);
        style.shadow = DROP_SHADOW;
        style.border = iced::Border::default()
            .rounded(iced::border::Radius::new(iced::Pixels::from(f32::INFINITY)));
        style
    })
}
