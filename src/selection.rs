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
    pub status: SelectionStatus,
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
    /// Processes an image
    pub fn process_image(&self, width: u32, height: u32, pixels: &[u8]) -> image::DynamicImage {
        #[expect(clippy::cast_possible_truncation, reason = "pixels must be integer")]
        #[expect(
            clippy::cast_sign_loss,
            reason = "selection has been normalized so height and width will be positive"
        )]
        image::DynamicImage::from(
            image::RgbaImage::from_raw(width, height, pixels.to_vec())
                .expect("Image handle stores a valid image"),
        )
        .crop_imm(
            self.rect.x as u32,
            self.rect.y as u32,
            self.rect.width as u32,
            self.rect.height as u32,
        )
    }

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
            status: SelectionStatus::default(),
        }
    }

    delegate! {
        to self.rect {
            /// The height and width of the selection
            pub fn size(self) -> Size;
            /// Get the position
            pub fn pos(self) -> Point;
            /// Top-left, top-right, bottom-left and bottom-right points
            pub fn corners(self) -> Corners;
            /// Whether this selection contains a given point
            pub fn contains(self, point: Point) -> bool;
            /// Position of the top left corner
            pub fn top_left(self) -> Point;
            /// Position of the top right corner
            pub fn top_right(self) -> Point;
            /// Position of the bottom right corner
            pub fn bottom_right(self) -> Point;
            /// Position of the bottom left corner
            pub fn bottom_left(self) -> Point;
        }
        #[allow(dead_code)]
        #[expr(self.rect = $; self)]
        to self.rect {
            /// Update the size of the rect
            pub fn with_size<F: FnOnce(Size) -> Size>(mut self, f: F) -> Self;
            /// Update the position of the top left corner
            pub fn with_pos<F: FnOnce(Point) -> Point>(mut self, f: F) -> Self;
            /// Update the selection's height
            pub fn with_height<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self;
            /// Update the selection's width
            pub fn with_width<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self;
            /// Update the x coordinate of the top left corner
            pub fn with_x<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self;
            /// Update the y coordinate of the top left corner
            pub fn with_y<F: FnOnce(f32) -> f32>(mut self, f: F) -> Self;
            /// Make sure the width and height is not negative
            pub fn norm(mut self) -> Self;
        }
        to self.status {
            /// The selection is currently being dragged
            pub const fn is_dragged(self) -> bool;
            /// The selection is not moving
            pub const fn is_idle(self) -> bool;
            /// The selection is being resized
            pub const fn is_resized(self) -> bool;
        }
    }
}
