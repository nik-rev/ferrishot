//! A `Selection` is the structure representing a selected area in the background image
use delegate::delegate;
use iced::widget::{Column, Row, Space, row, tooltip};
use iced::{Element, Length, Padding};
use iced::{Point, Rectangle, Size};

use crate::corners::Corners;
use crate::corners::SideOrCorner;
use crate::icon;
use crate::message::Message;
use crate::rectangle::RectangleExt;
use crate::theme::THEME;

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
    pub fn render_border(&self, frame: &mut iced::widget::canvas::Frame, color: iced::Color) {
        // Render the rectangle around the selection (the sides)
        frame.stroke_rectangle(
            self.pos(),
            self.size(),
            iced::widget::canvas::Stroke::default()
                .with_color(THEME.drop_shadow)
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

    /// Render icons around the selection border
    #[expect(
        clippy::cast_possible_truncation,
        reason = "we only care about the amount of items we can render at most"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "normalized, so width nor height will be negative"
    )]
    // TODO: Currently, this function does not handle the case where the selection has the
    // same size as the entire screen - so no icons can be rendered at all.
    //
    // We should add even more fallbacks so that it can render a little bit inside of the selection.
    pub fn render_icons<'a>(self, image_width: f32, image_height: f32) -> Element<'a, Message> {
        fn add_icons_until_there_is_at_least_n_of_them<'a, const MIN_ELEMENTS: usize>(
            mut icons: Vec<Element<'a, Message>>,
            mut iter: impl Iterator<Item = (Element<'a, Message>, &'static str)>,
            mut padding: f32,
            total_icons_positioned: &mut usize,
            tooltip_position: tooltip::Position,
        ) -> (Vec<Element<'a, Message>>, f32) {
            while icons.len() < MIN_ELEMENTS {
                if let Some((next, tooltip_str)) = iter.by_ref().next() {
                    icons.push(
                        crate::widgets::icon_tooltip(next, tooltip_str, tooltip_position).into(),
                    );
                    *total_icons_positioned += 1;
                    padding -= PX_PER_ICON / 2.0;
                } else {
                    break;
                }
            }
            (icons, padding)
        }

        fn position_icons_in_line<'a>(
            space_available: f32,
            tooltip_position: tooltip::Position,
            total_icons_positioned: &mut usize,
            mut icons_iter: impl Iterator<Item = (Element<'a, Message>, &'static str)>,
            icons_len: usize,
        ) -> (Vec<Element<'a, Message>>, f32) {
            let icons_left_to_position = icons_len - *total_icons_positioned;
            let icons_rendered_here =
                ((space_available / PX_PER_ICON) as usize).min(icons_left_to_position);
            *total_icons_positioned += icons_rendered_here;

            // we do this thing because we need to know exactly
            // how many elems we got. size_hint may be unreliable
            let mut icons = Vec::with_capacity(icons_rendered_here);
            for _ in 0..icons_rendered_here {
                if let Some((icon, tooltip_str)) = icons_iter.by_ref().next() {
                    icons.push(
                        crate::widgets::icon_tooltip(icon, tooltip_str, tooltip_position).into(),
                    );
                }
            }

            // if there is just 0 element it will take away the icon padding so it can be negative
            // ensure it is positive
            let space_used = (icons.len() as f32)
                .mul_add(PX_PER_ICON, -SPACE_BETWEEN_ICONS)
                .max(0.0);

            let padding = (space_available - space_used) / 2.0;

            (icons, padding)
        }

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
        const SPACE_BETWEEN_ICONS: f32 = 2.0;

        let icons = vec![
            (
                icon!(Fullscreen).on_press(Message::SelectFullScreen).into(),
                "Select entire monitor (F11)",
            ),
            (
                icon!(Clipboard).on_press(Message::CopyToClipboard).into(),
                "Copy to Clipboard (Enter)",
            ),
            (
                icon!(Save).on_press(Message::SaveScreenshot).into(),
                "Save Screenshot (Ctrl + S)",
            ),
            (icon!(Close).on_press(Message::Exit).into(), "Exit (Esc)"),
        ];

        let sel = self.norm();

        let is_enough_space_at_bottom =
            image_height - (sel.rect.y + sel.rect.height) > ICON_BUTTON_SIZE;
        let is_enough_space_at_right =
            image_width - (sel.rect.x + sel.rect.width) > ICON_BUTTON_SIZE;
        let is_enough_space_at_top = sel.rect.y > ICON_BUTTON_SIZE;
        let is_enough_space_at_left = sel.rect.x > ICON_BUTTON_SIZE;

        let icons_len = icons.len();
        let mut icons_iter = icons.into_iter();
        let mut total_icons_positioned = 0;

        // first position the icons on each side (bottom -> right -> top -> left)
        // (bottom_icons, mut bottom_padding)

        let bottom_icons = is_enough_space_at_bottom.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Bottom,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let right_icons = is_enough_space_at_right.then(|| {
            position_icons_in_line(
                sel.rect.height,
                tooltip::Position::Right,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let top_icons = is_enough_space_at_top.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Top,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let left_icons = is_enough_space_at_left.then(|| {
            position_icons_in_line(
                sel.rect.height,
                tooltip::Position::Left,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        // if we reach here, our selection is to small to nicely
        // render all of the icons so we must "stack" them somehow

        // for the 4 sides, combined they will fit at LEAST 8 icons (3 top 3 bottom 1 right 1 left)

        let bottom_icons = bottom_icons.map(|(bottom_icons, bottom_padding)| {
            add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                bottom_icons,
                &mut icons_iter,
                bottom_padding,
                &mut total_icons_positioned,
                tooltip::Position::Bottom,
            )
        });

        let top_icons = top_icons.map(|(top_icons, top_padding)| {
            add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                top_icons,
                &mut icons_iter,
                top_padding,
                &mut total_icons_positioned,
                tooltip::Position::Top,
            )
        });

        let left_icons = left_icons.map(|(left_icons, left_padding)| {
            add_icons_until_there_is_at_least_n_of_them::<MIN_SIDE_ICONS>(
                left_icons,
                &mut icons_iter,
                left_padding,
                &mut total_icons_positioned,
                tooltip::Position::Left,
            )
        });

        let right_icons = right_icons.map(|(right_icons, right_padding)| {
            add_icons_until_there_is_at_least_n_of_them::<MIN_SIDE_ICONS>(
                right_icons,
                &mut icons_iter,
                right_padding,
                &mut total_icons_positioned,
                tooltip::Position::Right,
            )
        });

        // position two additional rows of icons on top and bottom
        // if we STILL have extra icons left

        let extra_top_icons = is_enough_space_at_top.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Top,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let extra_bottom_icons = is_enough_space_at_bottom.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Bottom,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let extra_bottom_icons =
            extra_bottom_icons.map(|(extra_bottom_icons, extra_bottom_padding)| {
                add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                    extra_bottom_icons,
                    &mut icons_iter,
                    extra_bottom_padding,
                    &mut total_icons_positioned,
                    tooltip::Position::Bottom,
                )
            });

        let extra_top_icons = extra_top_icons.map(|(extra_top_icons, extra_top_padding)| {
            add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                extra_top_icons,
                &mut icons_iter,
                extra_top_padding,
                &mut total_icons_positioned,
                tooltip::Position::Top,
            )
        });

        let extra_extra_top_icons = is_enough_space_at_top.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Top,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let extra_extra_bottom_icons = is_enough_space_at_bottom.then(|| {
            position_icons_in_line(
                sel.rect.width,
                tooltip::Position::Bottom,
                &mut total_icons_positioned,
                &mut icons_iter,
                icons_len,
            )
        });

        let extra_extra_top_icons =
            extra_extra_top_icons.map(|(extra_extra_top_icons, extra_extra_top_padding)| {
                add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                    extra_extra_top_icons,
                    &mut icons_iter,
                    extra_extra_top_padding,
                    &mut total_icons_positioned,
                    tooltip::Position::Top,
                )
            });

        let extra_extra_bottom_icons = extra_extra_bottom_icons.map(
            |(extra_extra_bottom_icons, extra_extra_bottom_padding)| {
                add_icons_until_there_is_at_least_n_of_them::<MIN_TOP_BOTTOM_ICONS>(
                    extra_extra_bottom_icons,
                    &mut icons_iter,
                    extra_extra_bottom_padding,
                    &mut total_icons_positioned,
                    tooltip::Position::Bottom,
                )
            },
        );

        // debug_assert!(
        //     icons_iter.as_slice().is_empty(),
        //     "all icons have been rendered"
        // );

        let right_icons = right_icons.map(|(right_icons, right_padding)| {
            Column::from_vec(right_icons)
                .spacing(SPACE_BETWEEN_ICONS)
                .width(PX_PER_ICON)
                .padding(Padding::default().top(right_padding))
        });

        let left_icons = left_icons.map(|(left_icons, left_padding)| {
            Column::from_vec(left_icons)
                .spacing(SPACE_BETWEEN_ICONS)
                .width(PX_PER_ICON)
                .padding(Padding::default().top(left_padding))
        });

        // there is no way to get amount of children
        // from a Row. that would be prety useful
        let mut top_icon_rows_count = 0;
        let top_icons: Column<_> = extra_extra_top_icons
            .into_iter()
            .chain(extra_top_icons)
            .chain(top_icons)
            .filter_map(|(icons, padding)| {
                (!icons.is_empty()).then(|| {
                    top_icon_rows_count += 1;
                    row![
                        Space::with_width(sel.rect.x),
                        Row::from_vec(icons)
                            .spacing(SPACE_BETWEEN_ICONS)
                            .height(PX_PER_ICON)
                            .padding(Padding::default().left(padding))
                    ]
                    .into()
                })
            })
            .collect();

        let bottom_icons: Column<_> = bottom_icons
            .into_iter()
            .chain(extra_bottom_icons)
            .chain(extra_extra_bottom_icons)
            .filter_map(|(icons, padding)| {
                (!icons.is_empty()).then(|| {
                    row![
                        Space::with_width(sel.rect.x),
                        Row::from_vec(icons)
                            .spacing(SPACE_BETWEEN_ICONS)
                            .height(PX_PER_ICON)
                            .padding(Padding::default().left(padding))
                    ]
                    .into()
                })
            })
            .collect();

        // include the frame so the icons do not touch the frame
        let selection_height = FRAME_WIDTH.mul_add(2.0, sel.rect.height);

        // the left and right rows should be large enough to have at least 1 icon
        // always.
        let height_added = (PX_PER_ICON - selection_height).max(0.0);

        iced::widget::column![
            // just whitespace necessary to align the icons to the selection
            Space::with_height(Length::Fixed(
                (top_icon_rows_count as f32).mul_add(-PX_PER_ICON, sel.rect.y - height_added / 2.0)
            ))
            .width(Length::Fill),
            // top icon row
            top_icons,
            // right icon row + left icon row
            row![Space::with_width(sel.rect.x - PX_PER_ICON).height(Length::Fill),]
                .push_maybe(left_icons)
                .push(
                    Space::with_width(FRAME_WIDTH.mul_add(2.0, sel.rect.width))
                        .height(Length::Fill)
                )
                .push_maybe(right_icons)
                .padding(Padding::default().top(height_added / 2.0))
                .height(selection_height + height_added),
            // bottom icon row
            bottom_icons,
        ]
        .into()
    }
}
