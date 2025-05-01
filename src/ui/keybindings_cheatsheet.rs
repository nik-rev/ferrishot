//! Keybindings cheatsheet

use std::iter;

use iced::{
    Background, Color, Element,
    Length::Fill,
    Pixels, Point, Renderer, Size, Task, Theme, Vector,
    advanced::svg::Svg,
    widget::{
        button, canvas, column, container, horizontal_space, row, stack, svg, vertical_space,
    },
};

use crate::{
    icon,
    icons::Icon,
    rect::{PointExt, Side, SizeExt, VectorExt},
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
                //
                // The actual cheatsheet
                //
                container(
                    canvas(self)
                        .width(CANVAS_SIZE.width)
                        .height(CANVAS_SIZE.height)
                )
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::BLACK)),
                    ..Default::default()
                }),
                //
                // Close Button 'x' in the top right corner
                //
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

use Action::{Extend, Move, Shrink};
use Side::{Bottom, Left, Right, Top};

/// Size of each selection
const SEL_SIZE: f32 = 100.0;
/// How much space to put between the rendered selections
const SPACE_BETWEEN: f32 = 100.0;
/// How far away the new selection from the old selection should be
const OFFSET: f32 = 20.0;
/// Size of each arrow
const ARROW_ICON_SIZE: f32 = 18.0;
/// Draw the cheatsheet with selections this far away from 0,0
const TOP_LEFT_OFFSET: Vector = Vector::new(180.0, 180.0);
/// Estimated size of the canvas
const CANVAS_SIZE: Size = Size::new(
    TOP_LEFT_OFFSET.y
        + (SEL_SIZE * HORIZONTAL_LABELS.len() as f32
            + SPACE_BETWEEN * VERTICAL_LABELS.len() as f32),
    TOP_LEFT_OFFSET.x
        + (SEL_SIZE * HORIZONTAL_LABELS.len() as f32
            + SPACE_BETWEEN * VERTICAL_LABELS.len() as f32),
);
/// Each of the 4 sides modified
const SIDES: [Side; 4] = [Left, Right, Bottom, Top];
/// Keys required
const HORIZONTAL_LABELS: [&str; 4] = ["h or 🡰", "l or 🡲", "j or 🡳", "k or 🡱"];
/// Modifier for eacrh the above keys, and what action it takes
const VERTICAL_LABELS: [(&str, Action, [Side; 4]); 3] = [
    ("", Move, SIDES),
    ("shift", Extend, SIDES),
    ("ctrl", Shrink, SIDES),
];
/// Text size of each label
const LABEL_TEXT_SIZE: f32 = 18.0;

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
        use Icon::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};

        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // --- Column labels ---
        for (x, label) in iter::successors(Some(-SEL_SIZE / 2.0 + TOP_LEFT_OFFSET.x), |x| {
            Some(x + SPACE_BETWEEN + SEL_SIZE)
        })
        .zip(HORIZONTAL_LABELS)
        {
            frame.fill_text(canvas::Text {
                content: label.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: Color::WHITE,
                position: Point::new(25.0 + x, 15.0),
                shaping: iced::widget::text::Shaping::Advanced,
                ..Default::default()
            });
        }

        for (y_count, (mods_label, action, sides)) in VERTICAL_LABELS.into_iter().enumerate() {
            // --- Row labels ---
            frame.fill_text(canvas::Text {
                content: mods_label.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: Color::WHITE,
                position: Point::new(
                    15.0,
                    15.0 + TOP_LEFT_OFFSET.y
                        + (SPACE_BETWEEN + SEL_SIZE)
                            .mul_add(y_count as f32, -(SEL_SIZE / HORIZONTAL_LABELS.len() as f32)),
                ),
                ..Default::default()
            });

            for (x_count, side) in sides.into_iter().enumerate() {
                // There are 2 selections that we will draw.
                //
                // - the `old` selection is before the transformation happens, and is dim
                //   it gets drawn over by the `new` selection
                // - the `new` selection is after
                //
                // For instance, we press `j` and it moves our selection down.
                // We want to show where it was (`old`), and where it is now (`new`).

                // Center point of the `old` selection
                let center = Point::new(
                    (SPACE_BETWEEN + SEL_SIZE) * x_count as f32,
                    (SPACE_BETWEEN + SEL_SIZE) * y_count as f32,
                );

                // Top-left of the `old` selection
                let old_pos = center - Vector::diag(SEL_SIZE / 2.0) + TOP_LEFT_OFFSET;

                // the `Selection` uses `selection_frame` for the color.
                // do this to avoid having to create a new theme key and having a switch for
                // dark frame / light frame
                let dimmed_theme = crate::config::Theme {
                    selection_frame: self.theme.selection_frame.scale_alpha(0.3),
                    ..self.theme
                };

                // draw the old selection: Dimmed, represents what was before the action took place
                //
                // e.g. hit `j` to go down: this selection represents what it looked like
                // before we hit `j`
                Selection::new(old_pos, &dimmed_theme, false, None)
                    .with_size(|_| Size::square(SEL_SIZE))
                    .draw_border(&mut frame);

                // New selection: Light. represents the state of selection after an action took place
                //
                // e.g. hit `j` to go down: this selection represents what it looked liked after
                // we hit j
                let new_sel = Selection::new(old_pos, &self.theme, false, None)
                    .with_size(|_| Size::square(SEL_SIZE));

                // When arrow icon is created, what ever `Point` it is on, will actually
                // be its top-left corner.
                //
                // This makes sure that the `Point` is in the middle of the icon
                let center_icon =
                    |new: Selection| new.center() - Vector::diag(ARROW_ICON_SIZE / 2.0);

                let (new_sel, icon, icon_pos) = match side {
                    Top => match action {
                        Shrink => {
                            let new = new_sel.with_y(|y| y + OFFSET).with_height(|h| h - OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.top_center().with_x(|x| x - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_y(|y| y - OFFSET).with_height(|h| h + OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.top_center()
                                    .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                                    .with_y(|y| y - ARROW_ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new_sel.with_y(|y| y - OFFSET);
                            (new, ArrowUp, center_icon(new))
                        }
                    },
                    Right => match action {
                        Shrink => {
                            let new = new_sel.with_width(|w| w - OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.right_center()
                                    .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                                    .with_x(|x| x - ARROW_ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_width(|w| w + OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.right_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new_sel.with_x(|x| x + OFFSET);
                            (new, ArrowRight, center_icon(new))
                        }
                    },
                    Bottom => match action {
                        Shrink => {
                            let new = new_sel.with_height(|h| h - OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.bottom_center()
                                    .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                                    .with_y(|y| y - ARROW_ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_height(|h| h + OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.bottom_center().with_x(|x| x - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new_sel.with_y(|y| y + OFFSET);
                            (new, ArrowDown, center_icon(new))
                        }
                    },
                    Left => match action {
                        Shrink => {
                            let new = new_sel.with_x(|x| x + OFFSET).with_width(|w| w - OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.left_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_x(|x| x - OFFSET).with_width(|w| w + OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.left_center()
                                    .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                                    .with_x(|x| x - ARROW_ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new_sel.with_x(|x| x - OFFSET);
                            (new, ArrowLeft, center_icon(new))
                        }
                    },
                };

                // draw the icon arrow
                frame.draw_svg(
                    iced::Rectangle {
                        x: icon_pos.x,
                        y: icon_pos.y,
                        width: ARROW_ICON_SIZE,
                        height: ARROW_ICON_SIZE,
                    },
                    Svg::new(icon.svg()).color(Color::WHITE),
                );

                new_sel.draw_border(&mut frame);
                new_sel.draw_corners(&mut frame);
            }
        }

        vec![frame.into_geometry()]
    }
}
