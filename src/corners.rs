//! `Corners` represents the 4 vertices of a `iced::Rectangle`
use iced::{Point, Rectangle, mouse};

use crate::{CORNER_RADIUS, INTERACTION_AREA, SELECTION_COLOR};

/// The point that is currently being resized
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Top side
    Top,
    /// Right side
    Right,
    /// Bottom side
    Bottom,
    /// Left side
    Left,
}

impl Side {
    /// Obtain the appropriate mouse cursor for the given side
    pub const fn mouse_icon(self) -> mouse::Interaction {
        match self {
            Self::Top | Self::Bottom => mouse::Interaction::ResizingVertically,
            Self::Right | Self::Left => mouse::Interaction::ResizingHorizontally,
            Self::TopLeft | Self::BottomRight => mouse::Interaction::ResizingDiagonallyDown,
            Self::BottomLeft | Self::TopRight => mouse::Interaction::ResizingDiagonallyUp,
        }
    }
}

/// Corners of an `iced::Rectangle`
#[derive(Debug, Default, Clone, Copy)]
pub struct Corners {
    /// Top left corner
    pub top_left: Point,
    /// Top right corner
    pub top_right: Point,
    /// Bottom left corner
    pub bottom_left: Point,
    /// Bottom right corner
    pub bottom_right: Point,
}

impl Corners {
    /// Render the circles for each side
    pub fn render_circles(&self, frame: &mut iced::widget::canvas::Frame) {
        for circle in [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
        .map(|corner| iced::widget::canvas::Path::circle(corner, CORNER_RADIUS))
        {
            frame.fill(&circle, SELECTION_COLOR);
        }
    }

    /// Return the interaction side for a point, if exists
    pub fn side_at(&self, point: Point) -> Option<Side> {
        let top = Rectangle {
            x: self.top_left.x,
            y: self.top_left.y - INTERACTION_AREA / 2.,
            width: self.top_right.x - self.top_left.x,
            height: INTERACTION_AREA,
        };
        let bottom = Rectangle {
            x: self.bottom_left.x,
            y: self.bottom_left.y - INTERACTION_AREA / 2.,
            width: self.bottom_right.x - self.bottom_left.x,
            height: INTERACTION_AREA,
        };
        let left = Rectangle {
            x: self.top_left.x - INTERACTION_AREA / 2.,
            y: self.top_left.y,
            width: INTERACTION_AREA,
            height: self.bottom_left.y - self.top_left.y,
        };
        let right = Rectangle {
            x: self.top_right.x - INTERACTION_AREA / 2.,
            y: self.top_right.y,
            width: INTERACTION_AREA,
            height: self.bottom_right.y - self.top_right.y,
        };
        let top_left = Rectangle {
            x: self.top_left.x - INTERACTION_AREA / 2.,
            y: self.top_left.y - INTERACTION_AREA / 2.,
            width: INTERACTION_AREA,
            height: INTERACTION_AREA,
        };
        let top_right = Rectangle {
            x: self.top_right.x - INTERACTION_AREA / 2.,
            y: self.top_right.y - INTERACTION_AREA / 2.,
            width: INTERACTION_AREA,
            height: INTERACTION_AREA,
        };
        let bottom_left = Rectangle {
            x: self.bottom_left.x - INTERACTION_AREA / 2.,
            y: self.bottom_left.y - INTERACTION_AREA / 2.,
            width: INTERACTION_AREA,
            height: INTERACTION_AREA,
        };
        let bottom_right = Rectangle {
            x: self.bottom_right.x - INTERACTION_AREA / 2.,
            y: self.bottom_right.y - INTERACTION_AREA / 2.,
            width: INTERACTION_AREA,
            height: INTERACTION_AREA,
        };

        [
            // NOTE: the corners shall come first since the corners and sides will intersect
            (top_left, Side::TopLeft),
            (top_right, Side::TopRight),
            (bottom_left, Side::BottomLeft),
            (bottom_right, Side::BottomRight),
            // the sides will also intersect at the vertices, but that's fine since the vertices
            // will take priority
            (top, Side::Top),
            (right, Side::Right),
            (left, Side::Left),
            (bottom, Side::Bottom),
        ]
        .into_iter()
        .find_map(|(dir, side)| dir.contains(point).then_some(side))
    }
}
