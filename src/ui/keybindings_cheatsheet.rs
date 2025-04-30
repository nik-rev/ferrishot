//! Keybindings cheatsheet

use std::iter;

use iced::{
    Background, Color, Element,
    Length::Fill,
    Pixels, Point, Renderer, Task, Theme,
    advanced::{Text, svg::Svg},
    widget::{
        button, canvas, column, container, horizontal_space, row, stack, svg, vertical_space,
    },
};

use crate::{
    icon,
    icons::Icon,
    rect::{PointExt, Side},
    ui::selection::Selection,
};

/// Keybinding cheatsheet state
#[derive(Debug, Clone, Default)]
pub struct State {
    /// If the keybinding cheatsheet is open
    pub is_open: bool,
}

/// Keybindings cheatsheet message
#[derive(Debug, Clone)]
pub enum Message {
    /// Open the keybindings menu
    Open,
    /// Close the keybindings menu
    Close,
}

impl crate::message::Handler for Message {
    fn handle(self, app: &mut crate::App) -> Task<crate::Message> {
        match self {
            Self::Open => app.keybinding_cheatsheet.is_open = true,
            Self::Close => app.keybinding_cheatsheet.is_open = false,
        }

        Task::none()
    }
}

/// Show a cheatsheet for the default keybindings available in ferrishot
#[derive(Debug, Copy, Clone)]
pub struct KeybindingsCheatsheet {
    /// Theme of the app
    pub theme: crate::config::Theme,
}

impl KeybindingsCheatsheet {
    /// Show the keybinding cheatsheet
    pub fn view(self) -> Element<'static, crate::Message> {
        container(
            stack![
                container(canvas(self).width(Fill).height(Fill)).style(|_| container::Style {
                    background: Some(Background::Color(Color::BLACK)),
                    ..Default::default()
                }),
                column![
                    vertical_space().height(10.0),
                    row![
                        horizontal_space().width(Fill),
                        button(
                            icon!(Close)
                                .style(|_, _| svg::Style {
                                    color: Some(Color::WHITE)
                                })
                                .width(24.0)
                                .height(24.0)
                        )
                        .on_press(crate::Message::KeyCheatsheet(Message::Close))
                        .style(|_, _| button::Style {
                            background: Some(Background::Color(Color::TRANSPARENT)),
                            ..Default::default()
                        }),
                        horizontal_space().width(10.0)
                    ]
                ]
            ]
            .height(800.0)
            .width(1000.0),
        )
        .center(Fill)
        .into()
    }
}

/// Size of each selection
const SIZE: f32 = 100.0;

/// How much space to put between the rendered selections
const SPACE_BETWEEN: f32 = 100.0;

/// How far away the new selection from the old selection should be
const OFFSET: f32 = 20.0;

/// What to do for a given side
#[derive(Clone, Copy)]
enum Action {
    /// Shrink a side
    Shrink,
    /// Extend a side
    Extend,
    /// Move the entire selection in a direction
    Move,
}

impl canvas::Program<crate::Message> for KeybindingsCheatsheet {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        use Action::{Extend, Move, Shrink};
        use Icon::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
        use Side::{Bottom, Left, Right, Top};

        const SPACE_ADDED: f32 = 180.0;
        const ICON_SIZE: f32 = 18.0;

        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let sides = [Left, Right, Bottom, Top];

        for (x, label) in iter::successors(Some(-SIZE / 2.0 + SPACE_ADDED), |x| {
            Some(x + SPACE_BETWEEN + SIZE)
        })
        .zip(["h or 🡰", "l or 🡲", "j or 🡳", "k or 🡱"])
        {
            frame.fill_text(canvas::Text {
                content: label.into(),
                size: Pixels(18.0),
                color: Color::WHITE,
                position: Point::new(25.0 + x, 15.0),
                shaping: iced::widget::text::Shaping::Advanced,
                ..Default::default()
            });
        }

        for (y_count, (mods_label, action, sides)) in [
            ("", Move, sides),
            ("shift", Extend, sides),
            ("ctrl", Shrink, sides),
        ]
        .into_iter()
        .enumerate()
        {
            frame.fill_text(canvas::Text {
                content: mods_label.into(),
                size: Pixels(18.0),
                color: Color::WHITE,
                position: Point::new(
                    15.0,
                    15.0 + SPACE_ADDED
                        + (SPACE_BETWEEN + SIZE).mul_add(y_count as f32, -(SIZE / 4.0)),
                ),
                ..Default::default()
            });

            for (x_count, side) in sides.into_iter().enumerate() {
                let center = Point::new(
                    (SPACE_BETWEEN + SIZE) * x_count as f32,
                    (SPACE_BETWEEN + SIZE) * y_count as f32,
                );

                let old_x = center.x - SIZE / 2.0 + SPACE_ADDED;
                let old_y = center.y - SIZE / 2.0 + SPACE_ADDED;

                let dimmed_theme = crate::config::Theme {
                    selection_frame: self.theme.selection_frame.scale_alpha(0.3),
                    ..self.theme
                };
                let mut dimmed_sel = Selection::new(
                    iced::Point { x: old_x, y: old_y },
                    &dimmed_theme,
                    false,
                    None,
                );
                dimmed_sel.rect.width = SIZE;
                dimmed_sel.rect.height = SIZE;

                dimmed_sel.draw_border(&mut frame);

                // new selection
                let mut new =
                    Selection::new(iced::Point { x: old_x, y: old_y }, &self.theme, false, None);

                new.rect.width = SIZE;
                new.rect.height = SIZE;

                let center = |new: Selection| {
                    new.center()
                        .with_x(|x| x - ICON_SIZE / 2.0)
                        .with_y(|y| y - ICON_SIZE / 2.0)
                };

                let (sel, icon, icon_pos) = match side {
                    Top => match action {
                        Shrink => {
                            let new = new.with_y(|y| y + OFFSET).with_height(|h| h - OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.top_center().with_x(|x| x - ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new.with_y(|y| y - OFFSET).with_height(|h| h + OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.top_center()
                                    .with_x(|x| x - ICON_SIZE / 2.0)
                                    .with_y(|y| y - ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new.with_y(|y| y - OFFSET);
                            (new, ArrowUp, center(new))
                        }
                    },
                    Right => match action {
                        Shrink => {
                            let new = new.with_width(|w| w - OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.right_center()
                                    .with_y(|y| y - ICON_SIZE / 2.0)
                                    .with_x(|x| x - ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new.with_width(|w| w + OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.right_center().with_y(|y| y - ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new.with_x(|x| x + OFFSET);
                            (new, ArrowRight, center(new))
                        }
                    },
                    Bottom => match action {
                        Shrink => {
                            let new = new.with_height(|h| h - OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.bottom_center()
                                    .with_x(|x| x - ICON_SIZE / 2.0)
                                    .with_y(|y| y - ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new.with_height(|h| h + OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.bottom_center().with_x(|x| x - ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new.with_y(|y| y + OFFSET);
                            (new, ArrowDown, center(new))
                        }
                    },
                    Left => match action {
                        Shrink => {
                            let new = new.with_x(|x| x + OFFSET).with_width(|w| w - OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.left_center().with_y(|y| y - ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new.with_x(|x| x - OFFSET).with_width(|w| w + OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.left_center()
                                    .with_y(|y| y - ICON_SIZE / 2.0)
                                    .with_x(|x| x - ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new.with_x(|x| x - OFFSET);
                            (new, ArrowLeft, center(new))
                        }
                    },
                };

                frame.draw_svg(
                    iced::Rectangle {
                        x: icon_pos.x,
                        y: icon_pos.y,
                        width: ICON_SIZE,
                        height: ICON_SIZE,
                    },
                    Svg::new(icon.svg()).color(Color::WHITE),
                );

                sel.draw_border(&mut frame);
                sel.draw_corners(&mut frame);
            }
        }

        vec![frame.into_geometry()]
    }
}
