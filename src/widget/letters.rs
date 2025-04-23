//! Render letters around the screen

use std::iter;

use iced::{
    Color, Font, Point,
    font::Weight,
    widget::canvas::{self, Path, Stroke},
};

/// How many letters to draw vertically
const VERTICAL_COUNT: f32 = 5.0;
/// How many letters to draw horizontally
const HORIZONTAL_COUNT: f32 = 5.0;
/// Color of lines
const LINE_COLOR: Color = Color::WHITE;
/// where does `a` start?
const UNICODE_CODEPOINT_LOWERCASE_A_START: u32 = 97;

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
        (*x < width - box_width).then_some(x + box_width)
    }) {
        for y in iter::successors(Some(y_start), |y| {
            (*y < height - box_height).then_some(y + box_height)
        }) {
            let boxes_drawn = (x / box_width).mul_add(HORIZONTAL_COUNT, y / box_height) as u32;

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

        // draw horizontal lines
        frame.stroke(
            &Path::line(
                Point::new(x + line_offset, y_start),
                Point::new(x + line_offset, height),
            ),
            Stroke {
                style: LINE_COLOR.into(),
                width: line_width,
                ..Default::default()
            },
        );
    }

    // draw vertical lines
    for y in iter::successors(Some(y_start), |y| (*y < height).then_some(y + box_height)) {
        frame.stroke(
            &Path::line(
                Point::new(x_start, y + line_offset),
                Point::new(width, y + line_offset),
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
    frame.stroke(
        &Path::line(
            Point::new(width - line_offset, y_start),
            Point::new(width - line_offset, height),
        ),
        Stroke {
            style: LINE_COLOR.into(),
            width: line_width,
            ..Default::default()
        },
    );
    frame.stroke(
        &Path::line(
            Point::new(x_start, height - line_offset),
            Point::new(width, height - line_offset),
        ),
        Stroke {
            style: LINE_COLOR.into(),
            width: line_width,
            ..Default::default()
        },
    );
}

use crate::message::Message;

/// Letters
pub struct Letters;

impl canvas::Program<Message> for Letters {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
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

        draw_boxes(
            x_start,
            y_start,
            width,
            height,
            &mut frame,
            FontSize::Fixed(48.0),
            1.0,
        );
        draw_boxes(
            x_start,
            y_start,
            width / 5.0,
            height / 5.0,
            &mut frame,
            FontSize::Fixed(32.0),
            1.0,
        );
        draw_boxes(
            x_start,
            y_start,
            width / 25.0,
            height / 25.0,
            &mut frame,
            FontSize::Fill,
            0.2,
        );

        vec![frame.into_geometry()]
    }
}
