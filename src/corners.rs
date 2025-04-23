//! `Corners` represents the 4 vertices of a `iced::Rectangle`

use std::str::FromStr;

use iced::{Point, Rectangle, mouse};
use strum::IntoEnumIterator;

use crate::rectangle::RectangleExt as _;

/// Corner of a rectangle
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    knus::DecodeScalar,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Corner {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
}

/// Side of a rectangle
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    knus::DecodeScalar,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Side {
    /// Top side
    Top,
    /// Right side
    Right,
    /// Bottom side
    Bottom,
    /// Left side
    Left,
}

/// Where to resize / shrink / extend rectangle
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    knus::DecodeScalar,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::EnumIter,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Direction {
    /// Above
    Up,
    /// Below
    Down,
    /// To the left
    Left,
    /// To the right
    Right,
}

/// Side and corner
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SideOrCorner {
    /// One of the 4 sides of a rectangle
    Side(Side),
    /// One of the 4 corners of a rectangle
    Corner(Corner),
}

impl SideOrCorner {
    /// All the variants for the side or corner
    fn variants() -> String {
        Side::iter()
            .map(|side| -> &'static str { side.into() })
            .chain(Corner::iter().map(|side| -> &'static str { side.into() }))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl FromStr for SideOrCorner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Side::from_str(s).map_or_else(
            |_| {
                Corner::from_str(s).map_or_else(
                    |_| Err(format!("expected one of {}", Self::variants())),
                    |corner| Ok(Self::Corner(corner)),
                )
            },
            |side| Ok(Self::Side(side)),
        )
    }
}

/// A named place of the rectangle
impl Corner {
    /// # Arguments
    ///
    /// - `self`: The corner, next to which we are resizing
    /// - `initial_rect`: The rectangle before we started resizing it
    /// - `current_cursor_pos`: Current position of the cursor
    /// - `initial_cursor_pos`: Position of the cursor before we started
    ///   resizing the rectangle
    ///
    /// # Returns
    ///
    /// The resized rectangle. The corner opposite to `self` is guaranteed to
    /// remain in-place.
    pub fn resize_rect(self, initial_rect: Rectangle, dy: f32, dx: f32) -> Rectangle {
        match self {
            Self::TopLeft => initial_rect
                .with_y(|y| y + dy)
                .with_x(|x| x + dx)
                .with_width(|w| w - dx)
                .with_height(|h| h - dy),
            Self::TopRight => initial_rect
                .with_y(|y| y + dy)
                .with_width(|w| w + dx)
                .with_height(|h| h - dy),
            Self::BottomLeft => initial_rect
                .with_x(|x| x + dx)
                .with_width(|w| w - dx)
                .with_height(|h| h + dy),
            Self::BottomRight => initial_rect.with_width(|w| w + dx).with_height(|h| h + dy),
        }
    }
}

impl SideOrCorner {
    /// Obtain the appropriate mouse cursor for the given side
    pub const fn mouse_icon(self) -> mouse::Interaction {
        match self {
            Self::Side(side) => match side {
                Side::Top | Side::Bottom => mouse::Interaction::ResizingVertically,
                Side::Right | Side::Left => mouse::Interaction::ResizingHorizontally,
            },
            Self::Corner(corner) => match corner {
                Corner::TopLeft | Corner::BottomRight => mouse::Interaction::ResizingDiagonallyDown,
                Corner::TopRight | Corner::BottomLeft => mouse::Interaction::ResizingDiagonallyUp,
            },
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
    /// Finds the nearest corner to this point
    pub fn nearest_corner(&self, point: Point) -> (Point, Corner) {
        let corners = [
            (self.top_left, Corner::TopLeft),
            (self.top_right, Corner::TopRight),
            (self.bottom_left, Corner::BottomLeft),
            (self.bottom_right, Corner::BottomRight),
        ];

        corners
            .into_iter()
            .min_by(|(point_a, _), (point_b, _)| {
                point
                    .distance(*point_a)
                    .total_cmp(&point.distance(*point_b))
            })
            .expect("`corners` has 4 elements. It would only be a None if it had `0` elements")
    }

    /// Render the circles for each side
    pub fn render_circles(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        accent_color: iced::Color,
    ) {
        /// Radius of each of the 4 corner circles in the frame drawn around the selection
        const FRAME_CIRCLE_RADIUS: f32 = 6.0;

        for circle in [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
        .map(|corner| iced::widget::canvas::Path::circle(corner, FRAME_CIRCLE_RADIUS))
        {
            frame.fill(&circle, accent_color);
        }
    }

    /// Return the interaction side for a point, if exists
    pub fn side_at(&self, point: Point) -> Option<SideOrCorner> {
        /// Shadow to apply to elements
        /// The area around each side of the frame which allows that side to be hovered over and resized
        const FRAME_INTERACTION_AREA: f32 = 35.0;
        let top = Rectangle {
            x: self.top_left.x,
            y: self.top_left.y - FRAME_INTERACTION_AREA / 2.,
            width: self.top_right.x - self.top_left.x,
            height: FRAME_INTERACTION_AREA,
        };
        let bottom = Rectangle {
            x: self.bottom_left.x,
            y: self.bottom_left.y - FRAME_INTERACTION_AREA / 2.,
            width: self.bottom_right.x - self.bottom_left.x,
            height: FRAME_INTERACTION_AREA,
        };
        let left = Rectangle {
            x: self.top_left.x - FRAME_INTERACTION_AREA / 2.,
            y: self.top_left.y,
            width: FRAME_INTERACTION_AREA,
            height: self.bottom_left.y - self.top_left.y,
        };
        let right = Rectangle {
            x: self.top_right.x - FRAME_INTERACTION_AREA / 2.,
            y: self.top_right.y,
            width: FRAME_INTERACTION_AREA,
            height: self.bottom_right.y - self.top_right.y,
        };
        let top_left = Rectangle {
            x: self.top_left.x - FRAME_INTERACTION_AREA / 2.,
            y: self.top_left.y - FRAME_INTERACTION_AREA / 2.,
            width: FRAME_INTERACTION_AREA,
            height: FRAME_INTERACTION_AREA,
        };
        let top_right = Rectangle {
            x: self.top_right.x - FRAME_INTERACTION_AREA / 2.,
            y: self.top_right.y - FRAME_INTERACTION_AREA / 2.,
            width: FRAME_INTERACTION_AREA,
            height: FRAME_INTERACTION_AREA,
        };
        let bottom_left = Rectangle {
            x: self.bottom_left.x - FRAME_INTERACTION_AREA / 2.,
            y: self.bottom_left.y - FRAME_INTERACTION_AREA / 2.,
            width: FRAME_INTERACTION_AREA,
            height: FRAME_INTERACTION_AREA,
        };
        let bottom_right = Rectangle {
            x: self.bottom_right.x - FRAME_INTERACTION_AREA / 2.,
            y: self.bottom_right.y - FRAME_INTERACTION_AREA / 2.,
            width: FRAME_INTERACTION_AREA,
            height: FRAME_INTERACTION_AREA,
        };

        [
            // NOTE: the corners shall come first since the corners and sides will intersect
            (top_left, SideOrCorner::Corner(Corner::TopLeft)),
            (top_right, SideOrCorner::Corner(Corner::TopRight)),
            (bottom_left, SideOrCorner::Corner(Corner::BottomLeft)),
            (bottom_right, SideOrCorner::Corner(Corner::BottomRight)),
            // the sides will also intersect at the vertices, but that's fine since the vertices
            // will take priority
            (top, SideOrCorner::Side(Side::Top)),
            (right, SideOrCorner::Side(Side::Right)),
            (left, SideOrCorner::Side(Side::Left)),
            (bottom, SideOrCorner::Side(Side::Bottom)),
        ]
        .into_iter()
        .find_map(|(dir, side)| dir.contains(point).then_some(side))
    }
}
