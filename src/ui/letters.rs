//! Render letters around the screen

use std::iter;

use iced::{
    Color, Element, Event, Font, Length, Point,
    font::Weight,
    keyboard::Key,
    widget::{
        Action, Canvas,
        canvas::{self, Path, Stroke},
    },
};

/// How many letters to draw vertically
const VERTICAL_COUNT: f32 = 5.0;
/// How many letters to draw horizontally
const HORIZONTAL_COUNT: f32 = 5.0;
/// Color of lines
const LINE_COLOR: Color = Color::WHITE;
/// where does `a` start?
const UNICODE_CODEPOINT_LOWERCASE_A_START: u32 = 97;
/// A tiny error margin for doing less than / greater than calculations
const ERROR_MARGIN: f32 = 0.001;

/// How large the font should be
#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum FontSize {
    /// A fixed font size in pixels
    Fixed(f32),
    /// The font size will fill the entire area
    ///
    /// Use this when the font size will be very small so it needs to be easy to see
    Fill,
}

/// Draw letters in a box
fn draw_boxes(
    x_start: f32,
    y_start: f32,
    width: f32,
    height: f32,
    frame: &mut canvas::Frame,
    font_size: FontSize,
    line_width: f32,
) {
    // We need to offset drawing each line, otherwise it will draw *half* of the line at each side
    let line_offset = line_width / 2.0;

    // `box` = the box which contains a single letter
    let box_width = width / HORIZONTAL_COUNT;
    let box_height = height / VERTICAL_COUNT;

    for x in iter::successors(Some(x_start), |x| {
        (*x + ERROR_MARGIN < x_start + width - box_width).then_some(x + box_width)
    }) {
        for y in iter::successors(Some(y_start), |y| {
            (*y + ERROR_MARGIN < y_start + height - box_height).then_some(y + box_height)
        }) {
            let boxes_drawn = ((x - x_start) / box_width)
                .mul_add(HORIZONTAL_COUNT, (y - y_start) / box_height)
                .round() as u32;

            frame.fill_text(iced::widget::canvas::Text {
                content: char::from_u32(UNICODE_CODEPOINT_LOWERCASE_A_START + boxes_drawn)
                    .expect("valid utf8 character")
                    .to_string(),
                position: iced::Point {
                    x: x + box_width / 2.0 - line_offset,
                    y: y + box_height / 2.0 - line_offset,
                },
                font: {
                    let mut font = Font::MONOSPACE;
                    if font_size == FontSize::Fill {
                        font.weight = Weight::Bold;
                    }
                    font
                },
                color: iced::Color::WHITE,
                size: match font_size {
                    FontSize::Fixed(px) => px,
                    FontSize::Fill => box_height,
                }
                .into(),
                align_x: iced::alignment::Horizontal::Center,
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        // draw vertical lines
        frame.stroke(
            &Path::line(
                Point::new(x + line_offset, y_start),
                Point::new(x + line_offset, y_start + height),
            ),
            Stroke {
                style: LINE_COLOR.into(),
                width: line_width,
                ..Default::default()
            },
        );
    }

    // draw horizontal lines
    for y in iter::successors(Some(y_start), |y| {
        (*y + ERROR_MARGIN < y_start + height).then_some(y + box_height)
    }) {
        frame.stroke(
            &Path::line(
                Point::new(x_start, y + line_offset),
                Point::new(x_start + width, y + line_offset),
            ),
            Stroke {
                style: LINE_COLOR.into(),
                width: line_width,
                ..Default::default()
            },
        );
    }

    // draw 2 extra lines at the end of each axis, so we have
    // lines on each side of equal thickness and its nice and symmetrical

    // horizontal line at the end
    frame.stroke(
        &Path::line(
            Point::new(x_start + width - line_offset, y_start),
            Point::new(x_start + width - line_offset, y_start + height),
        ),
        Stroke {
            style: LINE_COLOR.into(),
            width: line_width,
            ..Default::default()
        },
    );
    // vertical line at the end
    frame.stroke(
        &Path::line(
            Point::new(x_start, y_start + height - line_offset),
            Point::new(x_start + width, y_start + height - line_offset),
        ),
        Stroke {
            style: LINE_COLOR.into(),
            width: line_width,
            ..Default::default()
        },
    );
}

use crate::message::Message;

/// The letter grid consists of 3 "levels"
///
/// - Level 1: the entire screen is divided into 25 regions, a letter is assigned to each
///   region. When we input a letter, 1 of the 25 regions is picked and we progress onto level 2.
/// - Level 2: The region that we picked is further divided into 25 smaller regions. A single letter
///   is assigned to each region once again. Inputting another letter progresses us to Level 3.
/// - Level 3: The region picked in Level 2 is further divided into 25 even tinier regions. Now, once we
///   pick any of the tiny regions the center of that region will be sent as a `Message` to the main
///   `App`.
///
/// On the first level
///
/// Level of the letter grid.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum LetterLevel {
    /// First level
    #[default]
    First,
    /// Second click on the letter grid
    /// Choose a more precise location than the first
    Second {
        /// top left corner of the region clicked during `First`
        point: Point,
    },
    /// Third click on the letter grid
    /// Once we click this, it's finished and we will notify the `App`
    Third {
        /// top left corner of the region clicked during `Second`
        point: Point,
    },
}

/// Pick a position for a corner in the rectangle
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
pub enum PickCorner {
    /// Picking position of the top-left corner of the selection
    TopLeft,
    /// Picking position for the bottom-right corner of the selection
    BottomRight,
}

/// Letters
#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Letters {
    /// Corner to pick the position for
    pub pick_corner: PickCorner,
}

impl Letters {
    /// Render a grid of letters
    pub fn view(self) -> Element<'static, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

/// State of the letters
#[derive(PartialEq, Clone, Default)]
pub struct LettersState {
    /// The level of selection for this letter
    level: LetterLevel,
}

impl canvas::Program<Message> for Letters {
    type State = LettersState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            bounds.position(),
            bounds.size(),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.6,
            },
        );

        let x_start = 0.0;
        let y_start = 0.0;
        let width = frame.width();
        let height = frame.height();

        match state.level {
            LetterLevel::First => draw_boxes(
                x_start,
                y_start,
                width,
                height,
                &mut frame,
                FontSize::Fixed(48.0),
                1.0,
            ),
            LetterLevel::Second { point } => draw_boxes(
                point.x,
                point.y,
                width / HORIZONTAL_COUNT,
                height / VERTICAL_COUNT,
                &mut frame,
                FontSize::Fixed(32.0),
                1.0,
            ),
            LetterLevel::Third { point } => draw_boxes(
                point.x,
                point.y,
                width / HORIZONTAL_COUNT.powi(2),
                height / VERTICAL_COUNT.powi(2),
                &mut frame,
                FontSize::Fill,
                0.2,
            ),
        };

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
            modified_key: Key::Character(input),
            ..
        }) = event
        {
            if let Some(ch) = input.chars().next() {
                let ch = ch as u32 - UNICODE_CODEPOINT_LOWERCASE_A_START;
                let vertical_steps = (ch % VERTICAL_COUNT as u32) as f32;
                let horizontal_steps = (ch / HORIZONTAL_COUNT as u32) as f32;
                match state.level {
                    LetterLevel::First => {
                        let box_width = bounds.width / HORIZONTAL_COUNT;
                        let box_height = bounds.height / VERTICAL_COUNT;

                        state.level = LetterLevel::Second {
                            point: Point {
                                x: horizontal_steps * box_width,
                                y: vertical_steps * box_height,
                            },
                        };

                        return Some(Action::request_redraw());
                    }
                    LetterLevel::Second { point } => {
                        let box_width = bounds.width / HORIZONTAL_COUNT.powi(2);
                        let box_height = bounds.height / VERTICAL_COUNT.powi(2);

                        state.level = LetterLevel::Third {
                            point: Point {
                                x: horizontal_steps.mul_add(box_width, point.x),
                                y: vertical_steps.mul_add(box_height, point.y),
                            },
                        };

                        return Some(Action::request_redraw());
                    }
                    LetterLevel::Third { point } => {
                        let box_width = bounds.width / HORIZONTAL_COUNT.powi(3);
                        let box_height = bounds.height / VERTICAL_COUNT.powi(3);

                        return Some(Action::publish(Message::LettersPick {
                            // INFO: We want the point to be in the center, unlike in the previous levels where
                            // we wanted the top-left corner
                            point: Point {
                                x: horizontal_steps.mul_add(box_width, point.x) + box_width / 2.0,
                                y: vertical_steps.mul_add(box_height, point.y) + box_height / 2.0,
                            },
                        }));
                    }
                }
            }
        } else if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
            ..
        }) = event
        {
            return Some(Action::publish(Message::LettersAbort));
        }

        Some(Action::capture())
    }
}
