//! Shows useful information when pressing F12

use iced::{
    Background, Element,
    Length::Fill,
    Theme,
    widget::{Column, column, container, horizontal_space, row, scrollable, text, vertical_space},
};

use crate::message::Message;

use super::App;

/// Space between the label and what it represents
const LABEL_SPACE: f32 = 25.0;

/// Debug overlay shows useful information when pressing F12
pub fn debug_overlay(app: &App) -> Element<Message> {
    let container_style = |_: &Theme| container::Style {
        text_color: Some(app.config.theme.debug_fg),
        background: Some(Background::Color(app.config.theme.debug_bg)),
        ..Default::default()
    };

    row![
        container(
            scrollable(
                column![
                    text("Selection").color(app.config.theme.debug_label),
                    vertical_space().height(LABEL_SPACE),
                ]
                .push_maybe(app.selection.map(|sel| text!("{sel:#?}")))
            )
            .width(400.0),
        )
        .style(container_style),
        container(
            scrollable(column![
                text("Screenshot").color(app.config.theme.debug_label),
                vertical_space().height(LABEL_SPACE),
                text!("{:#?}", app.image),
            ])
            .width(400.0),
        )
        .style(container_style),
        horizontal_space().width(Fill),
        container(
            scrollable(column![
                text("Latest Messages").color(app.config.theme.debug_label),
                app.logged_messages
                    .iter()
                    .rev()
                    .take(5)
                    .map(|message| text!("{message:#?}").into())
                    .collect::<Column<_>>()
            ])
            .width(400.0)
            .height(Fill),
        )
        .style(container_style)
    ]
    .width(Fill)
    .height(Fill)
    .into()
}
