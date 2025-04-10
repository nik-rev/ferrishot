use delegate::delegate;
use iced::{Point, Rectangle, Size};

/// The selected area of the desktop which will be captured
#[derive(Debug, Default, Copy, Clone)]
pub struct Selection {
    pub rect: Rectangle,
    /// The selection is currently being moved (hold left click + move)
    pub moving_selection: Option<SelectionStatus>,
}

impl Selection {
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

    pub fn contains(&self, point: Point) -> bool {
        self.normalize().contains(point)
    }

    /// Create selection with a size of zero
    pub fn new(point: Point) -> Self {
        Self {
            rect: Rectangle::new(point, Size::default()),
            moving_selection: None,
        }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.rect = Rectangle::new(self.position(), size);
        self
    }

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
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SelectionStatus {
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
