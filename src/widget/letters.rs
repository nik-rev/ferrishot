//! Render letters around the screen

use std::iter;

use iced::{
    Color, Point,
    widget::canvas::{self, Path, Stroke},
};

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
        /// How many letters to draw vertically
        const VERTICAL_COUNT: f32 = 5.0;
        /// How many letters to draw horizontally
        const HORIZONTAL_COUNT: f32 = 5.0;
        /// Width of the lines drawn
        const LINE_WIDTH: f32 = 1.0;
        /// Color of lines
        const LINE_COLOR: Color = Color::WHITE;
        /// We need to offset drawing each line, otherwise it will draw *half* of the line at each side
        const LINE_OFFSET: f32 = LINE_WIDTH / 2.0;

        let mut rect = canvas::Frame::new(renderer, bounds.size());
        rect.fill_rectangle(
            bounds.position(),
            bounds.size(),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.6,
            },
        );

        let x_step = rect.width() / HORIZONTAL_COUNT;
        let y_step = rect.height() / VERTICAL_COUNT;
        let width = rect.width();
        let height = rect.height();

        for x in iter::successors(Some(0.0), |x| (*x < width).then_some(x + x_step)) {
            for y in iter::successors(Some(0.0), |y| (*y < height).then_some(y + y_step)) {
                let boxes_drawn = (x / x_step).mul_add(HORIZONTAL_COUNT, y / y_step) as u32;
                rect.fill_text(iced::widget::canvas::Text {
                    content: char::from_u32(97 + boxes_drawn)
                        .expect("valid utf8 character")
                        .to_string(),
                    position: iced::Point {
                        x: x + x_step / 2.0 - LINE_OFFSET,
                        y: y + y_step / 2.0 - LINE_OFFSET,
                    },
                    color: iced::Color::WHITE,
                    size: 32.0.into(),
                    align_x: iced::alignment::Horizontal::Center,
                    align_y: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
            }

            // draw horizontal lines
            rect.stroke(
                &Path::line(
                    Point::new(x + LINE_OFFSET, 0.0),
                    Point::new(x + LINE_OFFSET, rect.height()),
                ),
                Stroke {
                    style: LINE_COLOR.into(),
                    width: LINE_WIDTH,
                    ..Default::default()
                },
            );
        }

        // draw vertical lines
        for y in iter::successors(Some(0.0), |y| (*y < height).then_some(y + y_step)) {
            rect.stroke(
                &Path::line(
                    Point::new(0.0, y + LINE_OFFSET),
                    Point::new(rect.width(), y + LINE_OFFSET),
                ),
                Stroke {
                    style: LINE_COLOR.into(),
                    width: LINE_WIDTH,
                    ..Default::default()
                },
            );
        }

        // draw 2 extra lines at the end of each axis, so we have
        // lines on each side of equal thickness and its nice and symmetrical
        rect.stroke(
            &Path::line(
                Point::new(width - LINE_OFFSET, 0.0),
                Point::new(width - LINE_OFFSET, rect.height()),
            ),
            Stroke {
                style: LINE_COLOR.into(),
                width: LINE_WIDTH,
                ..Default::default()
            },
        );
        rect.stroke(
            &Path::line(
                Point::new(0.0, height - LINE_OFFSET),
                Point::new(rect.width(), height - LINE_OFFSET),
            ),
            Stroke {
                style: LINE_COLOR.into(),
                width: LINE_WIDTH,
                ..Default::default()
            },
        );

        vec![rect.into_geometry()]
    }
}
