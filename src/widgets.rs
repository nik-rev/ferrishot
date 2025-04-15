//! Widgets with custom styles
use iced::{
    Background, Border, Element, Length, Rectangle, Shadow,
    widget::{self, Space, column, row},
};

use crate::{
    constants::{ICON_BUTTON_SIZE, ICON_SIZE},
    rectangle::RectangleExt as _,
    selection::selection_lock::SelectionIsSome,
    theme::THEME,
};

use crate::message::Message;

/// Renders a tiny numeric input which shows a dimension of the rect and allow resizing it
pub fn size_indicator<'a>(
    image_height: u32,
    image_width: u32,
    rect: Rectangle,
    sel_is_some: SelectionIsSome,
) -> Element<'a, Message> {
    fn dimension_indicator<'a>(
        value: u32,
        max: u32,
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
                    s.parse::<u32>()
                        .ok()
                        .filter(|x| *x <= max)
                        .map_or(Message::NoOp, &on_change)
                }
            })
            .style(|_, _| widget::text_input::Style {
                value: THEME.size_indicator_fg,
                selection: THEME.text_selection_bg,
                // --- none
                background: Background::Color(THEME.transparent),
                border: iced::Border {
                    color: THEME.transparent,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                icon: THEME.transparent,
                placeholder: THEME.transparent,
            })
            .padding(0.0);

        input
    }

    const SPACING: f32 = 12.0;
    const ESTIMATED_INDICATOR_WIDTH: u32 = 104;
    const ESTIMATED_INDICATOR_HEIGHT: u32 = 24;

    let x_offset =
        (rect.bottom_right().x + SPACING).min((image_width - ESTIMATED_INDICATOR_WIDTH) as f32);
    let y_offset =
        (rect.bottom_right().y + SPACING).min((image_height - ESTIMATED_INDICATOR_HEIGHT) as f32);

    let horizontal_space = Space::with_width(x_offset);
    let vertical_space = Space::with_height(y_offset);

    let width = dimension_indicator(rect.width as u32, image_width, move |new_width| {
        Message::ResizeHorizontally {
            new_width,
            sel_is_some,
        }
    });
    let height = dimension_indicator(rect.height as u32, image_height, move |new_height| {
        Message::ResizeVertically {
            new_height,
            sel_is_some,
        }
    });
    let x = iced::widget::text("x ").color(THEME.fg);
    let space = iced::widget::text(" ");
    let c = widget::container(row![space, width, x, height]).style(|_| widget::container::Style {
        text_color: None,
        background: Some(Background::Color(THEME.size_indicator_bg)),
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
    });

    column![vertical_space, row![horizontal_space, c]].into()
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
