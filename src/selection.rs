//! A `Selection` is the structure representing a selected area in the background image
use delegate::delegate;
use iced::widget::{Column, Row, Space, row, tooltip};
use iced::{Element, Length, Padding};
use iced::{Point, Rectangle, Size};

use crate::config::KeyAction;
use crate::corners::Corners;
use crate::corners::SideOrCorner;
use crate::message::Message;
use crate::rectangle::RectangleExt;
use crate::{CONFIG, icon};

/// The size of the lines of the frame of the selection
pub const FRAME_WIDTH: f32 = 2.0;

/// Size of the button for the icon, which includes the
/// icon itself and space around it (bigger than `ICON_SIZE`)
pub const ICON_BUTTON_SIZE: f32 = 37.0;

/// How fast the selection resizes
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Speed {
    /// Resize follows the cursor. Cursor moves 1px -> the selection resizes by 1px
    Regular,
    /// Resize is slower than the cursor. Cursor moves 1px -> the selection resizes by less than that
    Slow {
        /// The speed was previously different, so the selection status must be updated to sync
        has_speed_changed: bool,
    },
}

impl Speed {
    /// For a given px of cursor movement, how many px does the selection resize by?
    pub const fn speed(self) -> f32 {
        match self {
            Self::Regular => 1.0,
            Self::Slow { .. } => 0.1,
        }
    }
}

/// The selected area of the desktop which will be captured
#[derive(Debug, Default, Copy, Clone, PartialEq)]
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
    Resize {
        /// Position of the selection rectangle before we started resizing it
        initial_rect: Rectangle,
        /// Cursor position before we started resizing it
        initial_cursor_pos: Point,
        /// The side or corner being resized
        resize_side: SideOrCorner,
    },
    /// The selection is currently being moved entirely
    ///
    /// left click + hold + move mouse
    Move {
        /// Top-left point of the selection Rect before we started dragging the
        /// selection
        initial_rect_pos: Point,
        /// Position of the cursor when we just started dragging the selection
        initial_cursor_pos: Point,
    },
    /// The selection is currently being created, e.g.
    /// hold left click and drag
    Create,
    /// The selection is not moving
    #[default]
    Idle,
}

/// Methods for guarantee that selection exists
///
/// We have this because very often in the app we want to pass the knowledge that our `Selection`
/// exists through a `Message`, however it is not possible to do that
///
/// For example, we send `Message::Foo` from `<App as canvas::Program<Message>>::update` if, and only if `App.selection.is_some()`.
///
/// Inside of `App::update` we receive this message and we have access to a `&mut App`. We need to
/// modify the selection and we are certain that it exists. Yet we must still use an `unwrap`.
///
/// This module prevents that. When obtaining a `Selection` from an `App`, we also get a `SelectionIsSome`.
/// This struct is only possible to construct from the `Option<Selection>::get` method.
///
/// This adds a little bit of complexity in exchange for preventing dozens of `expect`/`unwrap`s in the app and a type-safe way of guaranteeing that `Selection` exists.
pub mod selection_lock {
    use super::Selection;

    /// The existance of this struct guarantees that an `Option<Selection>` is always `Some`.
    ///
    /// # Important
    ///
    /// This struct should *never* be created manually. It should only ever be obtained from the
    /// `Option<&mut Selection>::get` method.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SelectionIsSome {
        /// Private field makes this type impossible to construct outside of the module it is defined in
        _private: (),
    }

    /// Methods for extracting value from an optional selection,
    /// with a guarantee that it can never be None.
    #[easy_ext::ext(OptionalSelectionExt)]
    pub impl Option<Selection> {
        /// Attempt to get the inner selection. if successful, return a key that allows opening
        /// this option again with a guarantee for existance.
        fn get(self) -> Option<(Selection, SelectionIsSome)> {
            self.map(|x| (x, SelectionIsSome { _private: () }))
        }
        /// Extract the selection, with a guarantee that it is always there
        fn unlock(&mut self, _key: SelectionIsSome) -> &mut Selection {
            self.as_mut()
                .expect("Cannot be None if the key is provided")
        }
    }
}

impl Selection {
    /// Convert the image into its final form, with crop (and in the future will also have
    /// "decorations" such as arrow, circle, square)
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
    pub fn draw_border(&self, frame: &mut iced::widget::canvas::Frame, color: iced::Color) {
        // Render the rectangle around the selection (the sides)
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(CONFIG.theme.drop_shadow)
                .with_width(FRAME_WIDTH * 2.0),
        );
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(color)
                .with_width(FRAME_WIDTH),
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
            /// Top left corner of the selection
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
            pub const fn is_move(self) -> bool;
            /// The selection is not moving
            pub const fn is_idle(self) -> bool;
            /// The selection is being resized
            pub const fn is_resize(self) -> bool;
            /// The selection is being created
            pub const fn is_create(self) -> bool;
        }
    }
}
