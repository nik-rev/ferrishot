use iced::{Point, Rectangle, Size};

use crate::corners::Corners;

#[allow(dead_code)]
#[easy_ext::ext(RectangleExt)]
pub impl Rectangle<f32> {
    /// make sure that the top-left corner is ALWAYS in the top left
    /// (it could be that top-left corner is actually on the bottom right,
    /// and we have a negative width and height):
    ///
    ///                           ----------
    ///                           |        |
    ///                           |        | <- height: -3
    ///                           |        |
    /// our "top left" is here -> O---------
    /// even if the width and height is negative
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

    /// Update height and width
    fn with_size(mut self, size: Size) -> Self {
        self = Self::new(self.position(), size);
        self
    }

    /// Update size using a closure
    fn map_size<F: FnOnce(Size) -> Size>(self, f: F) -> Self {
        self.with_size(f(self.size()))
    }

    /// Update position of the top left corner
    fn with_position(mut self, pos: Point) -> Self {
        self = Self::new(pos, self.size());
        self
    }

    /// Update position using a closure
    fn map_position<F: FnOnce(Point) -> Point>(self, f: F) -> Self {
        self.with_position(f(self.position()))
    }

    /// Update the x coordinate
    fn with_x(self, x: f32) -> Self {
        self.with_position(Point { x, y: self.y })
    }

    /// Update the x coordinate using a closure
    fn map_x<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_x(f(self.x))
    }

    /// Update the height
    fn with_height(self, height: f32) -> Self {
        self.with_size(Size {
            width: self.size().width,
            height,
        })
    }

    /// Update the height using a closure
    fn map_height<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_height(f(self.height))
    }

    /// Update the width
    fn with_width(self, width: f32) -> Self {
        self.with_size(Size {
            width,
            height: self.size().height,
        })
    }

    /// Update the width using a closure
    fn map_width<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_width(f(self.width))
    }

    /// Update the y coordinate
    fn with_y(self, y: f32) -> Self {
        self.with_position(Point { x: self.x, y })
    }

    /// Update the y coordinate using a closure
    fn map_y<F: FnOnce(f32) -> f32>(self, f: F) -> Self {
        self.with_position(Point {
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
}
