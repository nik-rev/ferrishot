use delegate::delegate;
use iced::{Point, Rectangle, Size};

use crate::corners::Corners;

/// The selected area of the desktop which will be captured
#[derive(Debug, Default, Copy, Clone)]
pub struct Selection {
    pub rect: Rectangle,
    /// The selection is currently being moved (hold left click + move)
    pub selection_status: SelectionStatus,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SelectionStatus {
    /// The selection is currently being resized
    Resized { rect: Rectangle, cursor: Point },
    /// The selection is currently being dragged
    ///
    /// left click + hold + move mouse
    Dragged {
        /// Top-left point of the selection Rect before we started dragging the
        /// selection
        rect_position: Point,
        /// Position of the cursor when we just started dragging the selection
        cursor: Point,
    },
    /// The selection is not moving
    #[default]
    Idle,
}

impl SelectionStatus {
    pub fn is_idle(&self) -> bool {
        *self == SelectionStatus::Idle
    }

    pub fn is_dragged(&self) -> bool {
        matches!(self, SelectionStatus::Dragged { .. })
    }

    pub fn is_resized(&self) -> bool {
        matches!(self, SelectionStatus::Resized { .. })
    }
}

impl Selection {
    /// Renders border of the selection
    pub fn render_border(&self, frame: &mut iced::widget::canvas::Frame) {
        // Render the rectangle around the selection (the sides)
        frame.stroke_rectangle(
            self.position(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(crate::app::SELECTION_COLOR)
                .with_width(crate::app::STROKE_SIZE),
        );
    }

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
    fn normalize(&self) -> Rectangle {
        let mut rect = self.rect;
        if rect.width.is_sign_negative() {
            rect.x = rect.x + rect.width;
            rect.width = rect.width.abs();
        }
        if rect.height.is_sign_negative() {
            rect.y = rect.y + rect.height;
            rect.height = rect.height.abs();
        }
        rect
    }

    /// If this selection contains the given point
    pub fn contains(&self, point: Point) -> bool {
        self.normalize().contains(point)
    }

    /// Which side contains this point, if any?
    ///
    /// Used for resizing the selection
    // pub fn side_contains(&self, point: Point) -> Option<Side> {
    //     let rect = self.normalize();
    //     let corners = self.corners();

    //     // .contains(point)
    // }

    /// Obtain coordinates of the 4 corners of the Selection
    pub fn corners(&self) -> Corners {
        let rect = self.normalize();
        let top_left = rect.position();
        Corners {
            top_left,
            top_right: Point::new(top_left.x + rect.width, top_left.y),
            bottom_left: Point::new(top_left.x, top_left.y + rect.height),
            bottom_right: Point::new(top_left.x + rect.width, top_left.y + rect.height),
        }
    }

    /// Create selection at a point with a size of zero
    pub fn new(point: Point) -> Self {
        Self {
            rect: Rectangle::new(point, Size::default()),
            selection_status: SelectionStatus::default(),
        }
    }

    /// Update height and width
    pub fn with_size(mut self, size: Size) -> Self {
        self.rect = Rectangle::new(self.position(), size);
        self
    }

    /// Update position of the top left corner
    pub fn with_position(mut self, pos: Point) -> Self {
        self.rect = Rectangle::new(pos, self.size());
        self
    }

    /// The x-coordinate of the top left point
    pub fn x(&self) -> f32 {
        self.rect.x
    }

    /// The y-coordinate of the top left point
    pub fn y(&self) -> f32 {
        self.rect.y
    }

    delegate! {
        to self.rect {
            pub fn position(&self) -> Point;
            pub fn size(&self) -> Size;
        }
        to self.selection_status {
            pub fn is_dragged(&self) -> bool;
            pub fn is_idle(&self) -> bool;
            pub fn is_resized(&self) -> bool;
        }
    }
}
