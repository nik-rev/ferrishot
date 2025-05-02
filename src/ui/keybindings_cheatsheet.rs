//! Keybindings cheatsheet

use std::iter;

use iced::{
    Background, Color, Element, Font,
    Length::Fill,
    Pixels, Point, Rectangle, Renderer, Size, Task, Theme, Vector,
    advanced::{graphics::geometry, svg::Svg},
    font::{self, Family, Weight},
    widget::{
        self as w, button,
        canvas::{LineCap, LineJoin, Path, Stroke},
        column, container, horizontal_space, row, stack, svg,
        text::Shaping,
        vertical_space,
    },
};

use crate::{
    icon,
    icons::Icon,
    rect::{PointExt, RectangleExt, Side, SizeExt, VectorExt},
    ui::{grid::GridBuilder, selection::Selection},
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
                container(column![
                    w::canvas(self)
                        .width(BASIC_MOVEMENTS_SIZE.width * 2.0)
                        .height(BASIC_MOVEMENTS_SIZE.height + 400.0),
                ])
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
            .height(1800.0)
            .width(2000.0),
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
/// How far away the `new` selection from the `old` selection should be
const SEL_NEW_OLD_OFFSET: f32 = 20.0;
/// Size of each arrow
const ARROW_ICON_SIZE: f32 = 18.0;
/// Draw the cheatsheet with selections this far away from 0,0
const TOP_LEFT_OFFSET: Vector = Vector::new(180.0, 250.0);
/// Size of the heading for the basic movements
const BASIC_MOVEMENTS_HEADING_SIZE: f32 = 30.0;
/// Estimated size of the canvas for the basic movements
const BASIC_MOVEMENTS_SIZE: Size = Size::new(
    TOP_LEFT_OFFSET.x
        + (SEL_SIZE * HORIZONTAL_LABELS.len() as f32
            + SPACE_BETWEEN * (HORIZONTAL_LABELS.len() - 1) as f32),
    TOP_LEFT_OFFSET.y
        + BASIC_MOVEMENTS_HEADING_SIZE
        + (SEL_SIZE * VERTICAL_LABELS.len() as f32
            + (SPACE_BETWEEN * (VERTICAL_LABELS.len() - 1) as f32)),
);
/// Each of the 4 sides modified
const SIDES: [Side; 4] = [Left, Right, Bottom, Top];
/// Keys required
const HORIZONTAL_LABELS: [(&str, &str); 4] = [
    ("LEFT", "h or 🡰"),
    ("RIGHT", "l or 🡲"),
    ("DOWN", "j or 🡳"),
    ("UP", "k or 🡱"),
];
/// Modifier for eacrh the above keys, and what action it takes
const VERTICAL_LABELS: [(&str, &str, Action, [Side; 4]); 3] = [
    ("", "MOVE", Move, SIDES),
    ("shift", "EXTEND", Extend, SIDES),
    ("ctrl", "SHRINK", Shrink, SIDES),
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

/// Teleport the selected region to a specific place
#[derive(Clone)]
enum GotoPlace {
    /// X-center
    XCenter,
    /// Y-center
    YCenter,
    /// Center
    Center,
    /// Top Left
    TopLeft,
    /// Bottom Right
    BottomRight,
}

/// Where to go for the 3rd part
#[derive(Clone)]
enum Part3RectPlace {
    /// Goto a specific place
    GotoPlace(GotoPlace),
    /// A side in the rectangle
    Side(Side),
}

impl w::canvas::Program<crate::Message> for KeybindingsCheatsheet {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<w::canvas::Geometry> {
        use Icon::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};

        let mut frame = w::canvas::Frame::new(renderer, bounds.size());

        // the `Selection` uses `selection_frame` for the color.
        // do this to avoid having to create a new theme key and having a switch for
        // dark frame / light frame
        let theme_with_dimmed_selection = crate::config::Theme {
            selection_frame: self.theme.selection_frame.scale_alpha(0.3),
            ..self.theme
        };

        // --- Cheatsheet Part 2 ---
        //
        // Keys: hjkl, arrow keys with mods:
        // - Shift
        // - No mods
        // - Alt
        // - Control
        //
        // Actions:
        // - move selection in  direction
        // - extend in direction
        // - shrink in direction
        //
        //

        // --- heading ---
        frame.fill_text(w::canvas::Text {
            content: "Transform region by 1px:".into(),
            position: Point::new(TOP_LEFT_OFFSET.x + 30.0, TOP_LEFT_OFFSET.y - 240.0),
            color: Color::WHITE,
            font: Font::MONOSPACE,
            size: Pixels(30.0),
            ..Default::default()
        });

        // --- Column labels ---
        for (x, (label, arrow)) in
            iter::successors(Some(-SEL_SIZE / 2.0 + TOP_LEFT_OFFSET.x), |x| {
                Some(x + SPACE_BETWEEN + SEL_SIZE)
            })
            .zip(HORIZONTAL_LABELS)
        {
            frame.fill_text(w::canvas::Text {
                content: label.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: self.theme.selection_frame,
                position: Point::new(25.0 + x, TOP_LEFT_OFFSET.y - 160.0),
                font: Font {
                    weight: Weight::Bold,
                    family: Family::Monospace,
                    ..Default::default()
                },
                shaping: Shaping::Basic,
                ..Default::default()
            });
            frame.fill_text(w::canvas::Text {
                content: arrow.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: Color::WHITE,
                position: Point::new(25.0 + x, TOP_LEFT_OFFSET.y - 136.0),
                shaping: Shaping::Advanced,
                ..Default::default()
            });
        }

        for (y_count, (mods_label, mods_action, action, sides)) in
            VERTICAL_LABELS.into_iter().enumerate()
        {
            let label_y_offset = TOP_LEFT_OFFSET.y
                + BASIC_MOVEMENTS_HEADING_SIZE
                + (SPACE_BETWEEN + SEL_SIZE)
                    .mul_add(y_count as f32, -(SEL_SIZE / HORIZONTAL_LABELS.len() as f32));
            // --- Row labels ---
            frame.fill_text(w::canvas::Text {
                content: mods_label.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: Color::WHITE,
                position: Point::new(15.0, 24.0 + label_y_offset),
                ..Default::default()
            });
            frame.fill_text(w::canvas::Text {
                content: mods_action.into(),
                size: Pixels(LABEL_TEXT_SIZE),
                color: self.theme.selection_frame,
                position: Point::new(15.0, label_y_offset),
                font: Font {
                    weight: Weight::Bold,
                    family: Family::Monospace,
                    ..Default::default()
                },
                shaping: Shaping::Basic,
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
                let old_pos = center - Vector::diag(SEL_SIZE / 2.0)
                    + TOP_LEFT_OFFSET
                    + Vector::y(BASIC_MOVEMENTS_HEADING_SIZE);

                // draw the old selection: Dimmed, represents what was before the action took place
                //
                // e.g. hit `j` to go down: this selection represents what it looked like
                // before we hit `j`
                Selection::new(old_pos, &theme_with_dimmed_selection, false, None)
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
                            let new = new_sel
                                .with_y(|y| y + SEL_NEW_OLD_OFFSET)
                                .with_height(|h| h - SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.top_center().with_x(|x| x - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new_sel
                                .with_y(|y| y - SEL_NEW_OLD_OFFSET)
                                .with_height(|h| h + SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.top_center()
                                    .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                                    .with_y(|y| y - ARROW_ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new_sel.with_y(|y| y - SEL_NEW_OLD_OFFSET);
                            (new, ArrowUp, center_icon(new))
                        }
                    },
                    Right => match action {
                        Shrink => {
                            let new = new_sel.with_width(|w| w - SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.right_center()
                                    .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                                    .with_x(|x| x - ARROW_ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_width(|w| w + SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.right_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new_sel.with_x(|x| x + SEL_NEW_OLD_OFFSET);
                            (new, ArrowRight, center_icon(new))
                        }
                    },
                    Bottom => match action {
                        Shrink => {
                            let new = new_sel.with_height(|h| h - SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowUp,
                                new.bottom_center()
                                    .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                                    .with_y(|y| y - ARROW_ICON_SIZE),
                            )
                        }
                        Extend => {
                            let new = new_sel.with_height(|h| h + SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowDown,
                                new.bottom_center().with_x(|x| x - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Move => {
                            let new = new_sel.with_y(|y| y + SEL_NEW_OLD_OFFSET);
                            (new, ArrowDown, center_icon(new))
                        }
                    },
                    Left => match action {
                        Shrink => {
                            let new = new_sel
                                .with_x(|x| x + SEL_NEW_OLD_OFFSET)
                                .with_width(|w| w - SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowRight,
                                new.left_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
                            )
                        }
                        Extend => {
                            let new = new_sel
                                .with_x(|x| x - SEL_NEW_OLD_OFFSET)
                                .with_width(|w| w + SEL_NEW_OLD_OFFSET);
                            (
                                new,
                                ArrowLeft,
                                new.left_center()
                                    .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                                    .with_x(|x| x - ARROW_ICON_SIZE),
                            )
                        }
                        Move => {
                            let new = new_sel.with_x(|x| x - SEL_NEW_OLD_OFFSET);
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

        //
        // --- tip ---
        //
        frame.fill_text(w::canvas::Text {
            content: "TIP".into(),
            position: Point::new(55.0, BASIC_MOVEMENTS_SIZE.height),
            color: Color::WHITE,
            size: Pixels(20.0),
            font: Font {
                family: Family::Monospace,
                weight: Weight::Bold,
                ..Default::default()
            },
            ..Default::default()
        });
        frame.fill_text(w::canvas::Text {
            content: ": Hold ALT while doing any of the above to transform by 125px!".into(),
            position: Point::new(90.0, BASIC_MOVEMENTS_SIZE.height),
            color: Color::WHITE,
            size: Pixels(20.0),
            font: Font::MONOSPACE,
            ..Default::default()
        });

        // --- part 2

        let sel = Selection::new(
            Point::new(
                BASIC_MOVEMENTS_SIZE.width.mul_add(0.5, -(SEL_SIZE / 2.0)),
                BASIC_MOVEMENTS_SIZE.height + 40.0,
            ),
            &self.theme,
            false,
            None,
        )
        .with_size(|_| Size::square(SEL_SIZE));

        sel.draw_border(&mut frame);
        sel.draw_corners(&mut frame);

        let dotted_stroke = Stroke {
            style: w::canvas::Style::Solid(self.theme.selection_frame),
            width: 3.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            line_dash: w::canvas::LineDash {
                segments: &[5.0],
                offset: 0,
            },
        };

        let radius = 25.0;

        frame.stroke(&Path::circle(sel.top_left(), radius), dotted_stroke);
        frame.stroke(&Path::circle(sel.bottom_right(), radius), dotted_stroke);

        // --- heading ---
        frame.fill_text(w::canvas::Text {
            content: "Pick top and then bottom corners".into(),
            position: Point::new(160.0, BASIC_MOVEMENTS_SIZE.height + 100.0),
            color: Color::WHITE,
            size: Pixels(30.0),
            font: Font::MONOSPACE,
            ..Default::default()
        });
        // --- subheading ---
        frame.fill_text(w::canvas::Text {
            content: "select any area of the screen in 8 keystrokes!".into(),
            position: Point::new(180.0, BASIC_MOVEMENTS_SIZE.height + 100.0),
            color: Color::WHITE.scale_alpha(0.8),
            size: Pixels(20.0),
            font: Font::MONOSPACE,
            ..Default::default()
        });

        // --- top left label ---
        frame.fill_text(w::canvas::Text {
            content: "Pick top left corner: t".into(),
            position: sel.top_left() - Vector::new(200.0, 20.0),
            color: Color::WHITE,
            ..Default::default()
        });
        // --- bottom right label ---
        frame.fill_text(w::canvas::Text {
            content: "Pick bottom right corner: b".into(),
            position: sel.bottom_right() + Vector::x(50.0),
            color: Color::WHITE,
            ..Default::default()
        });

        GridBuilder::default()
            .top_left(Point::new(
                BASIC_MOVEMENTS_SIZE.width + TOP_LEFT_OFFSET.x - 40.0,
                TOP_LEFT_OFFSET.y - 40.0,
            ))
            .cell_size(Size::new(100.0, 100.0))
            .spacing(Size::new(90.0, 100.0))
            .columns(3)
            .cells(
                [
                    // Row 1
                    (Part3RectPlace::Side(Top), "gk", "go up as far\nas possible"),
                    (
                        Part3RectPlace::Side(Bottom),
                        "gj",
                        "go down as far\nas possible",
                    ),
                    (
                        Part3RectPlace::Side(Right),
                        "gl",
                        "go right as far\nas possible",
                    ),
                    // Row 2
                    (
                        Part3RectPlace::Side(Left),
                        "gh",
                        "go left as far\nas possible",
                    ),
                    (
                        Part3RectPlace::GotoPlace(GotoPlace::XCenter),
                        "gx",
                        "go to x-center",
                    ),
                    (
                        Part3RectPlace::GotoPlace(GotoPlace::YCenter),
                        "gy",
                        "go to y-center",
                    ),
                    // Row 3
                    (
                        Part3RectPlace::GotoPlace(GotoPlace::Center),
                        "gc",
                        "go to center",
                    ),
                    (
                        Part3RectPlace::GotoPlace(GotoPlace::TopLeft),
                        "gg",
                        "go to top left",
                    ),
                    (
                        Part3RectPlace::GotoPlace(GotoPlace::BottomRight),
                        "G",
                        "go to bottom right",
                    ),
                ]
                .iter()
                .map(|(item_action, key, description)| {
                    crate::ui::grid::CellBuilder::default()
                        .draw(move |frame: &mut w::canvas::Frame, bounds: Rectangle| {
                            let grid_cell_size = Size::new(100.0, 100.0);
                            let sel_size_in_grid_item = 40.0;

                            let sel_size = Size::square(sel_size_in_grid_item);
                            let origin = bounds.top_left();

                            let old_sel_pos = Point::new(
                                sel_size_in_grid_item.mul_add(-1.5, grid_cell_size.width),
                                sel_size_in_grid_item * 0.5,
                            ) + origin.into_vector();

                            let old_sel = Selection::new(
                                old_sel_pos,
                                &theme_with_dimmed_selection,
                                false,
                                None,
                            )
                            .with_size(|_| sel_size);

                            old_sel.draw_border(frame);

                            let mut new_sel = match item_action {
                                Part3RectPlace::GotoPlace(goto_place) => match goto_place {
                                    GotoPlace::XCenter => old_sel.with_x(|_| {
                                        origin.x + grid_cell_size.width / 2.0 - sel_size.width / 2.0
                                    }),
                                    GotoPlace::YCenter => old_sel.with_y(|_| {
                                        origin.y + grid_cell_size.height / 2.0
                                            - sel_size.height / 2.0
                                    }),
                                    GotoPlace::Center => old_sel.with_pos(|_| {
                                        Point::new(
                                            grid_cell_size.width / 2.0 - sel_size.width / 2.0,
                                            grid_cell_size.height / 2.0 - sel_size.height / 2.0,
                                        ) + origin.into_vector()
                                    }),
                                    GotoPlace::TopLeft => old_sel.with_pos(|_| origin),
                                    GotoPlace::BottomRight => old_sel.with_pos(|_| {
                                        Point::new(
                                            grid_cell_size.width - sel_size.width,
                                            grid_cell_size.height - sel_size.height,
                                        ) + origin.into_vector()
                                    }),
                                },
                                Part3RectPlace::Side(side) => match side {
                                    Top => old_sel.with_y(|_| origin.y),
                                    Right => old_sel.with_x(|_| {
                                        origin.x + grid_cell_size.width - sel_size.width
                                    }),
                                    Bottom => old_sel.with_y(|_| {
                                        origin.y + grid_cell_size.height - sel_size.height
                                    }),
                                    Left => old_sel.with_x(|_| origin.x),
                                },
                            };

                            new_sel.theme = self.theme;

                            new_sel.draw_border(frame);
                            new_sel.draw_corners(frame);
                        })
                        .stroke(Stroke {
                            style: geometry::Style::Solid(Color::WHITE),
                            width: 1.0,
                            line_cap: LineCap::Round,
                            line_join: LineJoin::Round,
                            line_dash: w::canvas::LineDash {
                                segments: &[10.0],
                                offset: 0,
                            },
                        })
                        .label(w::canvas::Text {
                            content: (*key).to_string(),
                            position: Point::new(0.0, 0.0),
                            color: Color::WHITE,
                            font: Font::MONOSPACE,
                            shaping: Shaping::Advanced,
                            ..Default::default()
                        })
                        .description(w::canvas::Text {
                            content: (*description).to_string(),
                            position: Point::new(0.0, 0.0),
                            color: self.theme.selection_frame,
                            font: Font {
                                family: Family::Monospace,
                                weight: Weight::Normal,
                                style: font::Style::Italic,
                                ..Default::default()
                            },
                            shaping: Shaping::Advanced,
                            ..Default::default()
                        })
                        .build()
                        .expect("valid build")
                })
                .collect::<Vec<_>>(),
            )
            .build()
            .expect("required arguments passed")
            .draw(&mut frame);

        vec![frame.into_geometry()]
    }
}
