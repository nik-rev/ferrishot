//! Additional extension methods for `iced::Rectangle`
use iced::{Point, Rectangle, Size};

use crate::corners::Corners;

/// Extension methods for `iced::Point`
#[allow(dead_code)]
#[easy_ext::ext(PointExt)]
pub impl Point<f32> {
    /// Set the x coordinate of the point
    fn set_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }
    /// Compute the new x coordinate of the point from a closure
    fn with_x<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self {
        self.x = f(self.x);
        self
    }
    /// Set the y coordinate of the point from a closure
    fn set_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }
    /// Compute the new y coordinate of the point from a closure
    fn with_y<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self {
        self.y = f(self.y);
        self
    }
}

/// Extension methods for `iced::Rectangle`
#[allow(dead_code)]
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
    fn normalize(mut self) -> Self {
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
        let rect = self.normalize();
        let top_left = rect.position();
        Corners {
            top_left,
            top_right: Point::new(top_left.x + rect.width, top_left.y),
            bottom_left: Point::new(top_left.x, top_left.y + rect.height),
            bottom_right: Point::new(top_left.x + rect.width, top_left.y + rect.height),
        }
    }

    /// Position of the top left corner
    fn top_left(&self) -> Point {
        self.position()
    }

    /// Position of the top right corner
    fn top_right(&self) -> Point {
        self.position().with_x(|x| x + self.width)
    }

    /// Position of the bottom right corner
    fn bottom_right(&self) -> Point {
        self.position()
            .with_x(|x| x + self.width)
            .with_y(|y| y + self.height)
    }

    /// Position of the bottom left corner
    fn bottom_left(&self) -> Point {
        self.position().with_y(|y| y + self.height)
    }

    /// Returns the position of the top left corner of the Rectangle
    fn pos(&self) -> Point {
        self.position()
    }

    /// Update height and width
    fn set_size(mut self, size: Size) -> Self {
        self = Self::new(self.position(), size);
        self
    }

    /// Update size using a closure
    fn with_size<F: FnOnce(Size) -> Size>(self, f: F) -> Self {
        self.set_size(f(self.size()))
    }

    /// Update position of the top left corner
    fn set_pos(mut self, pos: Point) -> Self {
        self = Self::new(pos, self.size());
        self
    }

    /// Update position using a closure
    fn with_pos<F: FnOnce(Point) -> Point>(self, f: F) -> Self {
        self.set_pos(f(self.position()))
    }

    /// Update the x coordinate
    fn set_x(self, x: f32) -> Self {
        self.set_pos(Point { x, y: self.y })
    }

    /// Update the x coordinate using a closure
    fn with_x<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.set_x(f(self.x))
    }

    /// Update the height
    fn set_height(self, height: f32) -> Self {
        self.set_size(Size {
            width: self.size().width,
            height,
        })
    }

    /// Update the height using a closure
    fn with_height<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.set_height(f(self.height))
    }

    /// Update the width
    fn set_width(self, width: f32) -> Self {
        self.set_size(Size {
            width,
            height: self.size().height,
        })
    }

    /// Update the width using a closure
    fn with_width<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.set_width(f(self.width))
    }

    /// Update the y coordinate
    fn set_y(self, y: f32) -> Self {
        self.set_pos(Point { x: self.x, y })
    }

    /// Update the y coordinate using a closure
    fn with_y<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.set_pos(Point {
            x: self.x,
            y: f(self.y),
        })
    }

    /// The x-coordinate of the top left point
    fn x(self) -> f32 {
        self.x
    }

    /// The y-coordinate of the top left point
    fn y(self) -> f32 {
        self.y
    }

    /// The width of rectangle
    fn width(self) -> f32 {
        self.width
    }

    /// The height of rectangle
    fn height(self) -> f32 {
        self.height
    }
}
