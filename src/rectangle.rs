//! Additional extension methods for `iced::Rectangle`
use iced::{Point, Rectangle, Size};

use crate::corners::Corners;

/// Extension methods for `iced::Point`
#[easy_ext::ext(PointExt)]
pub impl Point<f32> {
    /// Update the x coordinate of the point
    fn with_x<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self {
        self.x = f(self.x);
        self
    }

    /// Update the y coordinate of the point
    fn with_y<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self {
        self.y = f(self.y);
        self
    }
}

/// Extension methods for `iced::Rectangle`
#[easy_ext::ext(RectangleExt)]
pub impl Rectangle<f32> {
    /// make sure that the top-left corner is ALWAYS in the top left
    /// (it could be that top-left corner is actually on the bottom right,
    /// and we have a negative width and height):
    ///
    /// ```text
    ///                           ----------
    ///                           |        |
    ///                           |        | <- height: -3
    ///                           |        |
    /// our "top left" is here -> O---------
    /// even if the width and height is negative
    /// ```
    fn norm(mut self) -> Self {
        if self.width.is_sign_negative() {
            self.x += self.width;
            self.width = self.width.abs();
        }
        if self.height.is_sign_negative() {
            self.y += self.height;
            self.height = self.height.abs();
        }
        self
    }

    /// Obtain coordinates of the 4 corners of the Selection
    fn corners(self) -> Corners {
        let rect = self.norm();
        let top_left = rect.position();
        Corners {
            top_left,
            top_right: Point::new(top_left.x + rect.width, top_left.y),
            bottom_left: Point::new(top_left.x, top_left.y + rect.height),
            bottom_right: Point::new(top_left.x + rect.width, top_left.y + rect.height),
        }
    }

    /// Position of the top left corner
    fn pos(self) -> Point {
        self.position()
    }

    /// Position of the top left corner
    fn top_left(&self) -> Point {
        self.position()
    }

    /// Position of the top right corner
    fn top_right(&self) -> Point {
        self.top_left().with_x(|x| x + self.width)
    }

    /// Position of the bottom right corner
    fn bottom_right(&self) -> Point {
        self.top_left()
            .with_x(|x| x + self.width)
            .with_y(|y| y + self.height)
    }

    /// Position of the bottom left corner
    fn bottom_left(&self) -> Point {
        self.top_left().with_y(|y| y + self.height)
    }

    /// Update size of the rectangle
    fn with_size<F: FnOnce(Size) -> Size>(self, f: F) -> Self {
        Self::new(self.position(), f(self.size()))
    }

    /// Update the top left corner of the rectangle
    fn with_pos<F: FnOnce(Point) -> Point>(self, f: F) -> Self {
        Self::new(f(self.position()), self.size())
    }

    /// Update the x-coordinate
    fn with_x<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_pos(|_| Point {
            x: f(self.x),
            y: self.y,
        })
    }

    /// Update the height
    fn with_height<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_size(|_| Size {
            width: self.width,
            height: f(self.height),
        })
    }

    /// Update the width
    fn with_width<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_size(|_| Size {
            height: self.height,
            width: f(self.width),
        })
    }

    /// Update the y-coordinate of the top left corner
    fn with_y<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_pos(|_| Point {
            x: self.x,
            y: f(self.y),
        })
    }
}
