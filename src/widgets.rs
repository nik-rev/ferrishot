//! Widgets with custom styles
use iced::{Background, Border, Element, Length, Shadow, widget};

use crate::{
    constants::{ICON_BUTTON_SIZE, ICON_SIZE},
    theme::THEME,
};

use crate::message::Message;

/// Renders a tiny numeric input which shows a dimension of the rect and allow resizing it
pub fn size_indicator<'a>(
    label: &'static str,
    value: u32,
    max: u32,
    on_change: impl Fn(u32) -> Message + 'a,
) -> widget::Row<'a, Message> {
    let label = widget::text(label).color(THEME.size_indicator_fg);

    let content = value.to_string();
    let input = iced::widget::text_input(Default::default(), content.as_str())
        .on_input(move |s| {
            s.parse::<u32>()
                .ok()
                .filter(|x| *x <= max)
                .map_or(Message::None, &on_change)
        })
        .width(Length::Fixed((16 * content.len()) as f32))
        .style(|_, _| widget::text_input::Style {
            background: Background::Color(THEME.size_indicator_bg),
            value: THEME.size_indicator_fg,
            selection: THEME.text_selection_bg,
            // ---
            border: iced::Border {
                color: THEME.transparent,
                width: 0.0,
                radius: 0.0.into(),
            },
            icon: THEME.transparent,
            placeholder: THEME.transparent,
        })
        .padding(0.0);

    let label_px = widget::text("px").color(THEME.size_indicator_fg);

    widget::row![label, input, label_px]
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
