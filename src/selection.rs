//! A `Selection` is the structure representing a selected area in the background image
use delegate::delegate;
use iced::widget::{Column, Row, Space, row, tooltip};
use iced::{Element, Length, Padding};
use iced::{Point, Rectangle, Size};

use crate::constants::{FRAME_COLOR, SPACE_BETWEEN_ICONS};
use crate::constants::{FRAME_WIDTH, ICON_BUTTON_SIZE};
use crate::corners::Corners;
use crate::corners::SideOrCorner;
use crate::message::Message;
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
        /// The side or corner being resized
        resize_side: SideOrCorner,
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
    /// The selection is currently being created, e.g.
    /// hold left click and drag
    Create,
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
                .with_color(FRAME_COLOR)
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
            /// The selection is being resized
            pub const fn is_create(self) -> bool;
        }
    }

    /// Render icons around the selection border
    #[expect(
        clippy::cast_possible_truncation,
        reason = "we only care about the amount of items we can render at most"
    )]
    #[expect(
        clippy::cast_precision_loss,
        reason = "as we do not need to be precise"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "normalized, so width nor height will be negative"
    )]
    // TODO: this function contains a lot of duplication, the insides should be refactored
    // it guarantees to display a minimum of 22 icons even when the selection is very small
    // but this number needs to be updated in the future if we add more icons than that
    pub fn render_icons<'a>(
        self,
        icons: Vec<(Element<'a, Message>, &'static str)>,
    ) -> Element<'a, Message> {
        // Here is the behaviour that we want
        //
        // We have a list of icons we want to render.
        // We want to render every single one of them.
        // Each icon should not be shrunk, nor should it render in weird positions
        //
        // for each side in [bottom, right, top, left] we render
        // all of the icons that fit on that side.
        //
        // But then we may have a small selection which doesn't manage to render all of the icons
        const PX_PER_ICON: f32 = SPACE_BETWEEN_ICONS + ICON_BUTTON_SIZE;
        const MIN_TOP_BOTTOM_ICONS: usize = 3;
        const MIN_SIDE_ICONS: usize = 1;
        let sel = self.norm();
        let icons_len = icons.len();
        let mut icons_iter = icons.into_iter();
        let mut total_icons_positioned = 0;

        let mut position_icons_in_line = |space_available: f32,
                                          tooltip_position: tooltip::Position|
         -> (Vec<Element<'_, Message>>, f32) {
            let icons_left_to_position = icons_len - total_icons_positioned;
            let icons_rendered_here =
                ((space_available / PX_PER_ICON) as usize).min(icons_left_to_position);
            total_icons_positioned += icons_rendered_here;

            // we do this thing because we need to know exactly
            // how many elems we got. size_hint may be unreliable
            let mut icons = Vec::with_capacity(icons_rendered_here);
            for _ in 0..icons_rendered_here {
                if let Some((icon, tooltip_info)) = icons_iter.by_ref().next() {
                    icons.push(iced::widget::tooltip(icon, tooltip_info, tooltip_position).into());
                }
            }

            // if there is just 0 element it will take away the icon padding so it can be negative
            // ensure it is positive
            let space_used = (icons.len() as f32)
                .mul_add(PX_PER_ICON, -SPACE_BETWEEN_ICONS)
                .max(0.0);

            let padding = (space_available - space_used) / 2.0;

            (icons, padding)
        };

        // first position the icons on each side (bottom -> right -> top -> left)

        let (mut bottom_icons, mut bottom_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Bottom);
        let (mut right_icons, mut right_padding) =
            position_icons_in_line(sel.rect.height, tooltip::Position::Right);
        let (mut top_icons, mut top_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Top);
        let (mut left_icons, mut left_padding) =
            position_icons_in_line(sel.rect.height, tooltip::Position::Left);

        // if we reach here, our selection is to small to nicely
        // render all of the icons so we must "stack" them somehow

        // for the 4 sides, combined they will fit at LEAST 8 icons (3 top 3 bottom 1 right 1 left)

        while bottom_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                bottom_icons.push(
                    iced::widget::tooltip(next, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                bottom_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        while top_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                top_icons.push(
                    iced::widget::tooltip(next, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                top_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        while left_icons.len() < MIN_SIDE_ICONS {
            if let Some((icon, tooltip_str)) = icons_iter.by_ref().next() {
                left_icons.push(
                    iced::widget::tooltip(icon, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                left_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        while right_icons.len() < MIN_SIDE_ICONS {
            if let Some((icon, tooltip_str)) = icons_iter.by_ref().next() {
                right_icons.push(
                    iced::widget::tooltip(icon, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                right_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        // position two additional rows of icons on top and bottom
        // if we STILL have extra icons left

        let mut position_icons_in_line = |space_available: f32,
                                          tooltip_position: tooltip::Position|
         -> (Vec<Element<'_, Message>>, f32) {
            let icons_left_to_position = icons_len - total_icons_positioned;
            let icons_rendered_here =
                ((space_available / PX_PER_ICON) as usize).min(icons_left_to_position);
            total_icons_positioned += icons_rendered_here;

            // we do this thing because we need to know exactly
            // how many elems we got. size_hint may be unreliable
            let mut icons = Vec::with_capacity(icons_rendered_here);
            for _ in 0..icons_rendered_here {
                if let Some((icon, tooltip_info)) = icons_iter.by_ref().next() {
                    icons.push(iced::widget::tooltip(icon, tooltip_info, tooltip_position).into());
                }
            }

            // if there is just 0 element it will take away the icon padding so it can be negative
            // ensure it is positive
            let space_used = (icons.len() as f32)
                .mul_add(PX_PER_ICON, -SPACE_BETWEEN_ICONS)
                .max(0.0);

            let padding = (space_available - space_used) / 2.0;

            (icons, padding)
        };

        let (mut extra_top_icons, mut extra_top_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Top);
        let (mut extra_bottom_icons, mut extra_bottom_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Bottom);

        while extra_bottom_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                extra_bottom_icons.push(
                    iced::widget::tooltip(next, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                extra_bottom_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        while extra_top_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                extra_top_icons
                    .push(iced::widget::tooltip(next, tooltip_str, tooltip::Position::Top).into());
                total_icons_positioned += 1;
                extra_top_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }
        let mut position_icons_in_line = |space_available: f32,
                                          tooltip_position: tooltip::Position|
         -> (Vec<Element<'_, Message>>, f32) {
            let icons_left_to_position = icons_len - total_icons_positioned;
            let icons_rendered_here =
                ((space_available / PX_PER_ICON) as usize).min(icons_left_to_position);
            total_icons_positioned += icons_rendered_here;

            // we do this thing because we need to know exactly
            // how many elems we got. size_hint may be unreliable
            let mut icons = Vec::with_capacity(icons_rendered_here);
            for _ in 0..icons_rendered_here {
                if let Some((icon, tooltip_info)) = icons_iter.by_ref().next() {
                    icons.push(iced::widget::tooltip(icon, tooltip_info, tooltip_position).into());
                }
            }

            // if there is just 0 element it will take away the icon padding so it can be negative
            // ensure it is positive
            let space_used = (icons.len() as f32)
                .mul_add(PX_PER_ICON, -SPACE_BETWEEN_ICONS)
                .max(0.0);

            let padding = (space_available - space_used) / 2.0;

            (icons, padding)
        };

        let extra_top_icons_len = extra_bottom_icons.len();
        let extra_bottom_icons_len = extra_top_icons.len();

        let (mut extra_extra_top_icons, mut extra_extra_top_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Top);
        let (mut extra_extra_bottom_icons, mut extra_extra_bottom_padding) =
            position_icons_in_line(sel.rect.width, tooltip::Position::Bottom);

        while extra_extra_bottom_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                extra_extra_bottom_icons.push(
                    iced::widget::tooltip(next, tooltip_str, tooltip::Position::Bottom).into(),
                );
                total_icons_positioned += 1;
                extra_extra_bottom_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        while extra_extra_top_icons.len() < MIN_TOP_BOTTOM_ICONS {
            if let Some((next, tooltip_str)) = icons_iter.by_ref().next() {
                extra_extra_top_icons
                    .push(iced::widget::tooltip(next, tooltip_str, tooltip::Position::Top).into());
                total_icons_positioned += 1;
                extra_extra_top_padding -= PX_PER_ICON / 2.0;
            } else {
                break;
            }
        }

        let extra_extra_top_icons_len = extra_extra_bottom_icons.len();
        let extra_extra_bottom_icons_len = extra_extra_top_icons.len();

        debug_assert!(
            icons_iter.as_slice().is_empty(),
            "all icons have been rendered"
        );

        let bottom_icons = Row::from_vec(bottom_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(bottom_padding));
        let right_icons = Column::from_vec(right_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .width(PX_PER_ICON)
            .padding(Padding::default().top(right_padding));
        let top_icons = Row::from_vec(top_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(top_padding));
        let left_icons = Column::from_vec(left_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .width(PX_PER_ICON)
            .padding(Padding::default().top(left_padding));

        let extra_bottom_icons = Row::from_vec(extra_bottom_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(extra_bottom_padding));
        let extra_top_icons = Row::from_vec(extra_top_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(extra_top_padding));
        let extra_extra_bottom_icons = Row::from_vec(extra_extra_bottom_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(extra_extra_bottom_padding));
        let extra_extra_top_icons = Row::from_vec(extra_extra_top_icons)
            .spacing(SPACE_BETWEEN_ICONS)
            .height(PX_PER_ICON)
            .padding(Padding::default().left(extra_extra_top_padding));

        // include the frame so the icons do not touch the frame
        let selection_height = FRAME_WIDTH.mul_add(2.0, sel.rect.height);

        // the left and right rows should be large enough to have at least 1 icon
        // always.
        let height_added = (PX_PER_ICON - selection_height).max(0.0);

        iced::widget::column![
            // just whitespace necessary to align the icons to the selection
            Space::with_height(Length::Fixed(
                sel.rect.y
                    - PX_PER_ICON
                    - height_added / 2.0
                    - {
                        if extra_top_icons_len == 0 {
                            0f32
                        } else {
                            PX_PER_ICON
                        }
                    }
                    - {
                        if extra_extra_top_icons_len == 0 {
                            0f32
                        } else {
                            PX_PER_ICON
                        }
                    }
            ))
            .width(Length::Fill),
            // top icon row
            Column::new()
                .push_maybe(
                    (extra_extra_top_icons_len != 0)
                        .then(|| { row![Space::with_width(sel.rect.x), extra_extra_top_icons] })
                )
                .push_maybe(
                    (extra_top_icons_len != 0)
                        .then(|| { row![Space::with_width(sel.rect.x), extra_top_icons] })
                )
                .push(row![Space::with_width(sel.rect.x), top_icons]),
            // right icon row + left icon row
            row![
                Space::with_width(sel.rect.x - PX_PER_ICON).height(Length::Fill),
                left_icons,
                Space::with_width(FRAME_WIDTH.mul_add(2.0, sel.rect.width)).height(Length::Fill),
                right_icons
            ]
            .padding(Padding::default().top(height_added / 2.0))
            .height(selection_height + height_added),
            // bottom icon row
            Column::new()
                .push(row![Space::with_width(sel.rect.x), bottom_icons])
                .push_maybe(
                    (extra_bottom_icons_len != 0)
                        .then(|| { row![Space::with_width(sel.rect.x), extra_bottom_icons] })
                )
                .push_maybe(
                    (extra_extra_bottom_icons_len != 0)
                        .then(|| { row![Space::with_width(sel.rect.x), extra_extra_bottom_icons] })
                ),
        ]
        .into()
    }
}
