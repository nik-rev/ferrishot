//! Icon widgets

use iced::{Background, Border, Element, Length, Shadow, widget};

use crate::{CONFIG, widget::selection::ICON_BUTTON_SIZE};

/// Create a tooltip for an icon
pub fn icon_tooltip<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
    position: widget::tooltip::Position,
) -> widget::Tooltip<'a, Message> {
    widget::Tooltip::new(content, tooltip, position)
        .style(|_| widget::container::Style {
            text_color: Some(CONFIG.theme.tooltip_fg),
            background: Some(Background::Color(CONFIG.theme.tooltip_bg)),
            border: Border::default(),
            shadow: Shadow::default(),
        })
        .gap(10.0)
}

/// Styled icon as a button
pub fn icon<'a, Message>(icon: crate::icons::Icon) -> widget::Button<'a, Message> {
    /// Width and height for icons *inside* of buttons
    const ICON_SIZE: f32 = 32.0;

    widget::button(
        widget::Svg::new(icon.svg())
            .style(|_, _| widget::svg::Style {
                color: Some(CONFIG.theme.icon_fg),
            })
            .width(Length::Fixed(ICON_SIZE))
            .height(Length::Fixed(ICON_SIZE)),
    )
    .width(Length::Fixed(ICON_BUTTON_SIZE))
    .height(Length::Fixed(ICON_BUTTON_SIZE))
    .style(move |_, _| {
        let mut style = widget::button::Style::default().with_background(CONFIG.theme.icon_bg);
        style.shadow = Shadow {
            color: CONFIG.theme.drop_shadow,
            blur_radius: 3.0,
            offset: iced::Vector { x: 0.0, y: 0.0 },
        };
        style.border = iced::Border::default()
            .rounded(iced::border::Radius::new(iced::Pixels::from(f32::INFINITY)));
        style
    })
}
