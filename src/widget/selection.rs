//! A `Selection` is the structure representing a selected area in the background image
use crate::CONFIG;
use crate::corners::Corners;
use crate::corners::SideOrCorner;
use crate::message::Message;
use crate::rectangle::RectangleExt;
use delegate::delegate;
use iced::Element;
use iced::Length;
use iced::Renderer;
use iced::Theme;
use iced::mouse::Cursor;
use iced::mouse::Interaction;
use iced::widget::Action;
use iced::widget::Canvas;
use iced::widget::canvas;
use iced::{Point, Rectangle, Size};

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

/// The existance of this struct guarantees that an `Option<Selection>` is always `Some`.
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

impl Selection {
    /// Render the selection
    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

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

    /// Draw shade around the selection
    pub fn draw_shade(&self, frame: &mut iced::widget::canvas::Frame, image_bounds: Rectangle) {
        let sel = self.norm();

        // represents the area outside of the selection
        let outside = canvas::Path::new(|p| {
            p.move_to(image_bounds.top_left());
            p.line_to(image_bounds.top_right());
            p.line_to(image_bounds.bottom_right());
            p.line_to(image_bounds.bottom_left());
            p.move_to(image_bounds.top_left());

            p.move_to(sel.top_left());
            p.line_to(sel.bottom_left());
            p.line_to(sel.bottom_right());
            p.line_to(sel.top_right());
            p.move_to(sel.top_left());
        });

        frame.fill(&outside, CONFIG.theme.non_selected_region);
    }

    /// Renders border of the selection
    pub fn draw_border(&self, frame: &mut iced::widget::canvas::Frame) {
        // Draw the shadow of the border of the selection
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(CONFIG.theme.drop_shadow)
                .with_width(FRAME_WIDTH * 2.0),
        );
        // Draw the border around the selection (the sides)
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(CONFIG.theme.selection_frame)
                .with_width(FRAME_WIDTH),
        );
    }

    /// Render the circles for each side
    pub fn draw_corners(&self, frame: &mut iced::widget::canvas::Frame) {
        /// Radius of each of the 4 corner circles in the frame drawn around the selection
        const FRAME_CIRCLE_RADIUS: f32 = 6.0;

        let corners = self.corners();
        for circle in [
            corners.top_left,
            corners.top_right,
            corners.bottom_left,
            corners.bottom_right,
        ]
        .map(|corner| iced::widget::canvas::Path::circle(corner, FRAME_CIRCLE_RADIUS))
        {
            frame.fill(&circle, CONFIG.theme.selection_frame);
        }
    }

    /// Set status of the selection
    pub const fn with_status(mut self, status: SelectionStatus) -> Self {
        self.status = status;
        self
    }

    /// Create selection at a point with a size of zero
    pub fn new(point: Point) -> Self {
        Self {
            rect: Rectangle::new(point, Size::default()),
            status: SelectionStatus::default(),
        }
    }

    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    pub fn cursor_in_selection(self, cursor: Cursor) -> Option<(Point, Self)> {
        cursor.position().and_then(|cursor_pos| {
            self.norm()
                .contains(cursor_pos)
                .then_some((cursor_pos, self))
        })
    }

    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    pub fn cursor_in_selection_mut(&mut self, cursor: Cursor) -> Option<(Point, &mut Self)> {
        cursor.position().and_then(|cursor_pos| {
            self.norm()
                .contains(cursor_pos)
                .then_some((cursor_pos, self))
        })
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

/// Holds information about the mouse
#[derive(Default, Debug, Clone)]
pub struct SelectionKeysState {
    /// Left mouse click is currently being held down
    pub is_left_down: bool,
    /// Left mouse click is currently being held down
    pub is_right_down: bool,
    /// Shift key is currently being held down
    pub is_shift_down: bool,
}

impl canvas::Program<Message> for Selection {
    type State = SelectionKeysState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let sel = self.norm();

        sel.draw_shade(&mut frame, bounds);
        sel.draw_border(&mut frame);
        sel.draw_corners(&mut frame);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Interaction {
        let sel = self.norm();
        // if we are already resizing, then this cursor takes priority
        // e.g. we are resizing horizontally but we are on the top left
        // corner = we should have horizontal resize cursor.
        (if let SelectionStatus::Resize { resize_side, .. } = sel.status {
            Some(resize_side.mouse_icon())
        } else if sel.status.is_move() {
            Some(Interaction::Grabbing)
        } else {
            None
        })
        .or_else(|| {
            // when we started dragging a side, even if we go outside of the bounds of that side (which
            // happens often when we are dragging the mouse fast), we don't want the cursor to change
            cursor
                .position()
                .and_then(|cursor| sel.corners().side_at(cursor).map(SideOrCorner::mouse_icon))
        })
        .unwrap_or_else(|| {
            if self.cursor_in_selection(cursor).is_some() {
                Interaction::Grab
            } else {
                Interaction::Crosshair
            }
        })
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<Action<Message>> {
        use iced::Event::{Keyboard, Mouse};
        use iced::keyboard::Event::KeyPressed;
        use iced::keyboard::Event::KeyReleased;
        use iced::keyboard::Key::Named;
        use iced::keyboard::key::Named::Shift;
        use iced::mouse::Button::{Left, Right};
        use iced::mouse::Event::ButtonPressed;
        use iced::mouse::Event::ButtonReleased;
        use iced::mouse::Event::CursorMoved;

        let message = match event {
            Mouse(ButtonPressed(Left)) => {
                state.is_left_down = true;

                Message::LeftMouseDown(cursor)
            }
            Mouse(ButtonReleased(Left)) => {
                state.is_left_down = false;

                Message::EnterIdle
            }
            Keyboard(KeyReleased {
                key: Named(Shift), ..
            }) => {
                state.is_shift_down = false;
                return None;
            }
            Mouse(CursorMoved { position }) if self.is_resize() => {
                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Resize {
                    resize_side,
                    initial_rect,
                    initial_cursor_pos,
                } = self.status
                else {
                    unreachable!("has `.is_some_and(is_resized)` guard");
                };

                Message::Resize {
                    current_cursor_pos: *position,
                    resize_side,
                    initial_cursor_pos,
                    initial_rect,
                    sel_is_some: SelectionIsSome { _private: () },
                    speed: if state.is_shift_down {
                        Speed::Slow {
                            has_speed_changed: false,
                        }
                    } else {
                        Speed::Regular
                    },
                }
            }
            Keyboard(KeyPressed {
                key: Named(Shift), ..
            }) => {
                state.is_shift_down = true;

                let current_cursor_pos = cursor.position()?;

                // If we are already resizing a side, and we press shift, we
                // want to act as if we just started resizing from this point again
                // so we do not get a surprising jump
                match self.status {
                    SelectionStatus::Resize { resize_side, .. } => Message::Resize {
                        resize_side,
                        // start resizing from this point on
                        current_cursor_pos,
                        initial_cursor_pos: current_cursor_pos,
                        // the current selection becomes the new starting point
                        initial_rect: self.rect,
                        sel_is_some: SelectionIsSome { _private: () },
                        speed: Speed::Slow {
                            has_speed_changed: true,
                        },
                    },
                    SelectionStatus::Move { .. } => Message::MoveSelection {
                        current_cursor_pos,
                        initial_cursor_pos: current_cursor_pos,
                        current_selection: *self,
                        initial_rect_pos: self.pos(),
                        speed: Speed::Slow {
                            has_speed_changed: true,
                        },
                    },
                    _ => return None,
                }
            }
            Mouse(CursorMoved { position }) if self.is_move() => {
                let current_selection = self.norm();

                // FIXME: this will not be necessary when we have `if_let_guard`
                let SelectionStatus::Move {
                    initial_rect_pos,
                    initial_cursor_pos,
                } = current_selection.status
                else {
                    unreachable!();
                };

                Message::MoveSelection {
                    current_cursor_pos: *position,
                    initial_cursor_pos,
                    current_selection,
                    initial_rect_pos,
                    speed: if state.is_shift_down {
                        Speed::Slow {
                            has_speed_changed: false,
                        }
                    } else {
                        Speed::Regular
                    },
                }
            }
            Mouse(ButtonPressed(Right)) => {
                state.is_right_down = true;

                Message::ResizeToCursor {
                    cursor_pos: cursor.position()?,
                    selection: self.norm(),
                    sel_is_some: SelectionIsSome { _private: () },
                }
            }
            Mouse(ButtonReleased(Right)) => {
                state.is_right_down = false;

                Message::EnterIdle
            }
            Mouse(CursorMoved { position }) if self.is_create() => {
                Message::ExtendNewSelection(*position)
            }
            _ => return None,
        };

        Some(Action::publish(message))
    }
}
