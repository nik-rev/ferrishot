//! Keybindings cheatsheet

use iced::{
    Background, Color, Element, Font,
    Length::Fill,
    Pixels, Point, Rectangle, Renderer, Size, Task, Theme, Vector,
    advanced::{graphics::geometry, svg::Svg},
    font::{self, Family, Weight},
    widget::{
        self as w, button,
        canvas::{LineCap, LineJoin, Stroke},
        column, container, horizontal_space, row, stack, svg,
        text::Shaping,
        vertical_space,
    },
};

use crate::{
    icon,
    icons::Icon,
    rect::{PointExt, RectangleExt, Side, SizeExt, VectorExt},
    ui::{grid::Grid, selection::Selection},
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

/// Applies a transformation to the old selection, yielding the new selection
/// after some movement
type SelectionTransformer =
    fn(origin: Point, sel_size: Size, cell_size: Size, old_sel: Selection) -> Selection;

/// Type alias for the cell definition tuple for clarity
type CellDefinition = (
    fn(Selection) -> Selection, // Transform function
    Icon,                       // Icon enum variant
    fn(Selection) -> Point,     // Icon position function
);

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
        let mut frame = w::canvas::Frame::new(renderer, bounds.size());

        let dimmed_theme = crate::config::Theme {
            selection_frame: self.theme.selection_frame.scale_alpha(0.3),
            ..self.theme
        };

        let cell_definitions: [CellDefinition; 12] = [
            // --- MOVE ---
            (
                // Left
                |sel| sel.with_x(|x| x - SEL_NEW_OLD_OFFSET),
                Icon::ArrowLeft,
                |new_sel| new_sel.center() - Vector::diag(ARROW_ICON_SIZE / 2.0),
            ),
            (
                // Right
                |sel| sel.with_x(|x| x + SEL_NEW_OLD_OFFSET),
                Icon::ArrowRight,
                |new_sel| new_sel.center() - Vector::diag(ARROW_ICON_SIZE / 2.0),
            ),
            (
                // Down
                |sel| sel.with_y(|y| y + SEL_NEW_OLD_OFFSET),
                Icon::ArrowDown,
                |new_sel| new_sel.center() - Vector::diag(ARROW_ICON_SIZE / 2.0),
            ),
            (
                // Up
                |sel| sel.with_y(|y| y - SEL_NEW_OLD_OFFSET),
                Icon::ArrowUp,
                |new_sel| new_sel.center() - Vector::diag(ARROW_ICON_SIZE / 2.0),
            ),
            // --- EXTEND ---
            (
                // Left
                |sel| {
                    sel.with_x(|x| x - SEL_NEW_OLD_OFFSET)
                        .with_width(|w| w + SEL_NEW_OLD_OFFSET)
                },
                Icon::ArrowLeft,
                |new_sel| {
                    new_sel
                        .left_center()
                        .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                        .with_x(|x| x - ARROW_ICON_SIZE)
                },
            ),
            (
                // Right
                |sel| sel.with_width(|w| w + SEL_NEW_OLD_OFFSET),
                Icon::ArrowRight,
                |new_sel| new_sel.right_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
            ),
            (
                // Bottom
                |sel| sel.with_height(|h| h + SEL_NEW_OLD_OFFSET),
                Icon::ArrowDown,
                |new_sel| {
                    new_sel
                        .bottom_center()
                        .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                },
            ),
            (
                // Top
                |sel| {
                    sel.with_y(|y| y - SEL_NEW_OLD_OFFSET)
                        .with_height(|h| h + SEL_NEW_OLD_OFFSET)
                },
                Icon::ArrowUp,
                |new_sel| {
                    new_sel
                        .top_center()
                        .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                        .with_y(|y| y - ARROW_ICON_SIZE)
                },
            ),
            // --- SHRINK ---
            (
                // Left
                |sel| {
                    sel.with_x(|x| x + SEL_NEW_OLD_OFFSET)
                        .with_width(|w| w - SEL_NEW_OLD_OFFSET)
                },
                Icon::ArrowRight,
                |new_sel| new_sel.left_center().with_y(|y| y - ARROW_ICON_SIZE / 2.0),
            ),
            (
                // Right
                |sel| sel.with_width(|w| w - SEL_NEW_OLD_OFFSET),
                Icon::ArrowLeft,
                |new_sel| {
                    new_sel
                        .right_center()
                        .with_y(|y| y - ARROW_ICON_SIZE / 2.0)
                        .with_x(|x| x - ARROW_ICON_SIZE)
                },
            ),
            (
                // Bottom
                |sel| sel.with_height(|h| h - SEL_NEW_OLD_OFFSET),
                Icon::ArrowUp,
                |new_sel| {
                    new_sel
                        .bottom_center()
                        .with_x(|x| x - ARROW_ICON_SIZE / 2.0)
                        .with_y(|y| y - ARROW_ICON_SIZE)
                },
            ),
            (
                // Top
                |sel| {
                    sel.with_y(|y| y + SEL_NEW_OLD_OFFSET)
                        .with_height(|h| h - SEL_NEW_OLD_OFFSET)
                },
                Icon::ArrowDown,
                |new_sel| new_sel.top_center().with_x(|x| x - ARROW_ICON_SIZE / 2.0),
            ),
        ];

        let cells = cell_definitions
            .into_iter()
            .map(|(transform_func, icon, icon_pos_func)| {
                // Capture data specific to this cell definition for the closure
                // Note: Function pointers are Copy. Icons might need Clone if not Copy.
                // Themes need to be cloned if not Copy and captured.

                crate::ui::grid::Cell::builder()
                    .draw(move |frame: &mut w::canvas::Frame, bounds: Rectangle| {
                        // --- Cell Drawing Logic (Relative Coordinates) ---
                        let old_pos_relative = bounds.center() - Vector::diag(SEL_SIZE / 2.0);

                        let old_sel = Selection::new(old_pos_relative, &dimmed_theme, false, None)
                            .with_size(|_| Size::square(SEL_SIZE));

                        let new_sel_base =
                            Selection::new(old_pos_relative, &self.theme, false, None)
                                .with_size(|_| Size::square(SEL_SIZE));

                        // Apply the transformation specific to this cell
                        let new_sel = transform_func(new_sel_base);

                        // Calculate the icon position based on the *new* selection's geometry
                        let icon_pos_relative = icon_pos_func(new_sel); // Function calculates relative pos

                        // --- Draw elements relative to cell bounds ---
                        old_sel.draw_border(frame); // Draw dimmed border

                        frame.draw_svg(
                            iced::Rectangle {
                                x: icon_pos_relative.x,
                                y: icon_pos_relative.y,
                                width: ARROW_ICON_SIZE,
                                height: ARROW_ICON_SIZE,
                            },
                            Svg::new(icon.svg()).color(Color::WHITE), // Assuming icon.svg() exists
                        );

                        new_sel.draw_border(frame);
                        new_sel.draw_corners(frame); // Assuming draw_corners exists
                    })
                    .build()
            })
            .collect::<Vec<_>>();

        let grid = Grid::builder()
            .top_left(Point::new(180.0, 250.0))
            .cell_size(Size::square(SEL_SIZE))
            .spacing(Size::square(100.0))
            .columns(4)
            .title(geometry::Text {
                content: "Transform region by 1px:".to_string(),
                color: Color::WHITE,
                font: Font::MONOSPACE,
                size: Pixels(30.0),
                shaping: Shaping::Advanced,
                ..Default::default()
            })
            .dbg()
            .description(geometry::Text {
                content: "TIP: Hold ALT while doing any of the above to transform by 125px!"
                    .to_string(),
                color: Color::WHITE,
                size: Pixels(20.0),
                font: Font::MONOSPACE,
                shaping: Shaping::Advanced,
                ..Default::default()
            })
            .row_labels(vec![
                "MOVE".to_string(),
                "shift\nEXTEND".to_string(),
                "ctrl\nSHRINK".to_string(),
            ])
            .col_labels(vec![
                "LEFT\nh or 🡰".to_string(),
                "RIGHT\nl or 🡲".to_string(),
                "DOWN\nj or 🡳".to_string(),
                "UP\nk or 🡱".to_string(),
            ])
            .cells(cells)
            .build();

        let grid_size = grid.size();

        grid.draw(&mut frame);

        let cell_data: &[(&str, &str, SelectionTransformer)] = &[
            (
                "gk",
                "go up as far\nas possible",
                |origin, _, _, old_sel| old_sel.with_y(|_| origin.y),
            ),
            (
                "gj",
                "go down as far\nas possible",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_y(|_| origin.y + cell_size.height - sel_size.height)
                },
            ),
            (
                "gl",
                "go right as far\nas possible",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_x(|_| origin.x + cell_size.width - sel_size.width)
                },
            ),
            (
                "gh",
                "go left as far\nas possible",
                |origin, _, _, old_sel| old_sel.with_x(|_| origin.x),
            ),
            (
                "gx",
                "go to x-center",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_x(|_| origin.x + cell_size.width / 2.0 - sel_size.width / 2.0)
                },
            ),
            (
                "gy",
                "go to y-center",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_y(|_| origin.y + cell_size.height / 2.0 - sel_size.height / 2.0)
                },
            ),
            (
                "gc",
                "go to center",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_pos(|_| {
                        Point::new(
                            cell_size.width / 2.0 - sel_size.width / 2.0,
                            cell_size.height / 2.0 - sel_size.height / 2.0,
                        ) + origin.into_vector()
                    })
                },
            ),
            ("gg", "go to top left", |origin, _, _, old_sel| {
                old_sel.with_pos(|_| origin)
            }),
            (
                "G",
                "go to bottom right",
                |origin, sel_size, cell_size, old_sel| {
                    old_sel.with_pos(|_| {
                        Point::new(
                            cell_size.width - sel_size.width,
                            cell_size.height - sel_size.height,
                        ) + origin.into_vector()
                    })
                },
            ),
        ];

        frame.stroke_rectangle(
            Point::new(0.0, 0.0) + TOP_LEFT_OFFSET,
            grid_size,
            Stroke {
                style: geometry::Style::Solid(iced::color!(0xff_00_00)),
                width: 2.0,
                ..Default::default()
            },
        );

        Grid::builder()
            .top_left(Point::new(
                grid_size.width + TOP_LEFT_OFFSET.x * 2.0,
                TOP_LEFT_OFFSET.y,
            ))
            .cell_size(Size::new(100.0, 100.0))
            .spacing(Size::new(90.0, 100.0))
            .title(w::canvas::Text {
                content: "title of the grid...".to_string(),
                size: 30.0.into(),
                color: Color::WHITE,
                font: Font::MONOSPACE,
                shaping: Shaping::Advanced,
                ..Default::default()
            })
            .dbg()
            .columns(3)
            .cells(
                cell_data
                    .iter()
                    .map(|(key, desc, transform_old_sel)| {
                        crate::ui::grid::Cell::builder()
                            .draw(move |frame: &mut w::canvas::Frame, bounds: Rectangle| {
                                let cell_size = Size::new(100.0, 100.0);
                                let sel_size = Size::square(40.0);
                                let origin = bounds.top_left();

                                let old_pos = Point::new(
                                    (-1.5f32).mul_add(sel_size.width, cell_size.width),
                                    0.5 * sel_size.height,
                                ) + origin.into_vector();

                                let old_sel = Selection::new(old_pos, &dimmed_theme, false, None)
                                    .with_size(|_| sel_size);

                                let new_sel =
                                    transform_old_sel(origin, sel_size, cell_size, old_sel)
                                        .with_theme(self.theme);

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
                                color: Color::WHITE,
                                font: Font::MONOSPACE,
                                shaping: Shaping::Advanced,
                                ..Default::default()
                            })
                            .description(w::canvas::Text {
                                content: (*desc).to_string(),
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
                    })
                    .collect::<Vec<_>>(),
            )
            .build()
            .draw(&mut frame);
        // --- part 2

        // let sel = Selection::new(
        //     Point::new(
        //         BASIC_MOVEMENTS_SIZE.width.mul_add(0.5, -(SEL_SIZE / 2.0)),
        //         BASIC_MOVEMENTS_SIZE.height + 40.0,
        //     ),
        //     &self.theme,
        //     false,
        //     None,
        // )
        // .with_size(|_| Size::square(SEL_SIZE));

        // sel.draw_border(&mut frame);
        // sel.draw_corners(&mut frame);

        // let dotted_stroke = Stroke {
        //     style: w::canvas::Style::Solid(self.theme.selection_frame),
        //     width: 3.0,
        //     line_cap: LineCap::Round,
        //     line_join: LineJoin::Round,
        //     line_dash: w::canvas::LineDash {
        //         segments: &[5.0],
        //         offset: 0,
        //     },
        // };

        // let radius = 25.0;

        // frame.stroke(&Path::circle(sel.top_left(), radius), dotted_stroke);
        // frame.stroke(&Path::circle(sel.bottom_right(), radius), dotted_stroke);

        // // --- heading ---
        // frame.fill_text(w::canvas::Text {
        //     content: "Pick top and then bottom corners".into(),
        //     position: Point::new(160.0, BASIC_MOVEMENTS_SIZE.height + 100.0),
        //     color: Color::WHITE,
        //     size: Pixels(30.0),
        //     font: Font::MONOSPACE,
        //     ..Default::default()
        // });
        // // --- subheading ---
        // frame.fill_text(w::canvas::Text {
        //     content: "select any area of the screen in 8 keystrokes!".into(),
        //     position: Point::new(180.0, BASIC_MOVEMENTS_SIZE.height + 100.0),
        //     color: Color::WHITE.scale_alpha(0.8),
        //     size: Pixels(20.0),
        //     font: Font::MONOSPACE,
        //     ..Default::default()
        // });

        // // --- top left label ---
        // frame.fill_text(w::canvas::Text {
        //     content: "Pick top left corner: t".into(),
        //     position: sel.top_left() - Vector::new(200.0, 20.0),
        //     color: Color::WHITE,
        //     ..Default::default()
        // });
        // // --- bottom right label ---
        // frame.fill_text(w::canvas::Text {
        //     content: "Pick bottom right corner: b".into(),
        //     position: sel.bottom_right() + Vector::x(50.0),
        //     color: Color::WHITE,
        //     ..Default::default()
        // });

        vec![frame.into_geometry()]
    }
}
