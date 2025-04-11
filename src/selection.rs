//! A `Selection` is the structure representing a selected area in the background image
use delegate::delegate;
use iced::{Point, Rectangle, Size};

use crate::corners::Corners;
use crate::corners::Side;
use crate::rectangle::RectangleExt;

/// The selected area of the desktop which will be captured
#[derive(Debug, Default, Copy, Clone)]
pub struct Selection {
    /// Area represented by the selection
    pub rect: Rectangle,
    /// Status of the selection
    pub selection_status: SelectionStatus,
}

/// What the selection is doing at the moment
#[derive(Debug, Default, Clone, Copy, PartialEq, derive_more::IsVariant)]
pub enum SelectionStatus {
    /// The selection is currently being resized
    Resized {
        /// Position of the selection rectangle before we started resizing it
        initial_rect: Rectangle,
        /// Cursor position before we started resizing it
        initial_cursor_pos: Point,
        /// The side being resized
        resize_side: Side,
    },
    /// The selection is currently being dragged
    ///
    /// left click + hold + move mouse
    Dragged {
        /// Top-left point of the selection Rect before we started dragging the
        /// selection
        initial_rect_pos: Point,
        /// Position of the cursor when we just started dragging the selection
        initial_cursor_pos: Point,
    },
    /// The selection is not moving
    #[default]
    Idle,
}

impl Selection {
    /// Renders border of the selection
    pub fn render_border(&self, frame: &mut iced::widget::canvas::Frame) {
        // Render the rectangle around the selection (the sides)
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(crate::SELECTION_COLOR)
                .with_width(crate::STROKE_SIZE),
        );
    }

    /// Create selection at a point with a size of zero
    pub fn new(point: Point) -> Self {
        Self {
            rect: Rectangle::new(point, Size::default()),
            selection_status: SelectionStatus::default(),
        }
    }

    delegate! {
        to self.rect {
            /// The position of the top left corner of the selection
            #[call(position)]
            pub fn pos(self) -> Point;
            /// The height and width of the selection
            pub fn size(self) -> Size;
            /// Top-left, top-right, bottom-left and bottom-right points
            pub fn corners(self) -> Corners;
            /// x-coordinate of the top left corner of the selection
            pub fn x(self) -> f32;
            /// y-coordinate of the top left corner of the selection
            pub fn y(self) -> f32;
            /// Whether this selection contains a given point
            pub fn contains(self, point: Point) -> bool;
        }
        #[allow(dead_code)]
        #[expr(self.rect = $; self)]
        to self.rect {
            /// Set the size of the of the selection
            pub fn set_size(mut self, size: Size) -> Self;
            /// Compute the size of the selection from a closure
            pub fn with_size<F: FnOnce(Size) -> Size>(mut self, size: F) -> Self;

            /// Compute the position of the top point of the selection
            pub fn set_pos(mut self, pos: Point) -> Self;
            /// Compute the position of the top point of the selection from a closure
            pub fn with_pos<F: FnOnce(Point) -> Point>(mut self, pos: F) -> Self;

            /// Set the x coordinate of the top point of the selection
            pub fn set_x(mut self, x: f32) -> Self;
            /// Compute the x coordinate of the top point of the selection from a closure
            pub fn with_x<F: FnOnce(f32) -> f32>(mut self, x: F) -> Self;

            /// Set the x coordinate of the top point of the selection
            pub fn set_height(mut self, height: f32) -> Self;
            /// Compute the x coordinate of the top point of the selection from a closure
            pub fn with_height<F: FnOnce(f32) -> f32>(mut self, height: F) -> Self;

            /// Set the x coordinate of the top point of the selection
            pub fn set_width(mut self, width: f32) -> Self;
            /// Compute the x coordinate of the top point of the selection from a closure
            pub fn with_width<F: FnOnce(f32) -> f32>(mut self, width: F) -> Self;

            /// Set the y coordinate of the top point of the selection
            pub fn set_y(mut self, y: f32) -> Self;
            /// Compute the y coordinate of the top point of the selection from a closure
            pub fn with_y<F: FnOnce(f32) -> f32>(mut self, y: F) -> Self;

            /// Make sure the width and height is not negative
            pub fn normalize(mut self) -> Self;
        }
        to self.selection_status {
            /// The selection is currently being dragged
            pub const fn is_dragged(self) -> bool;
            /// The selection is not moving
            pub const fn is_idle(self) -> bool;
            /// The selection is being resized
            pub const fn is_resized(self) -> bool;
        }
    }
}
