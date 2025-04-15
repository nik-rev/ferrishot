//! A message represents some event in the app that mutates the state

use iced::{Point, Rectangle, mouse::Cursor};

use crate::{
    corners::SideOrCorner,
    selection::{Selection, selection_lock::SelectionIsSome},
};

/// Represents an action happening in the application
#[derive(Debug, Clone)]
pub enum Message {
    /// do nothing
    None,
    /// Change the height of the selection, bottom right does not move
    ResizeVertically {
        new_height: u32,
        sel_is_some: SelectionIsSome,
    },
    /// Change the width of the selection, bottom right does not move
    ResizeHorizontally {
        new_width: u32,
        sel_is_some: SelectionIsSome,
    },
    /// Exits the application
    Exit,
    /// The left mouse button is down
    LeftMouseDown(Cursor),
    /// Enter idle mode
    EnterIdle,
    /// Copy the screenshot to the clipboard
    CopyToClipboard,
    /// Save the screenshot as an image
    SaveScreenshot,
    /// The selection is initially resized as it was created
    InitialResize {
        /// Current position of the cursor
        current_cursor_pos: Point,
        /// Initial position of the cursor
        initial_cursor_pos: Point,
        /// Which side we are currently resizing
        resize_side: SideOrCorner,
        /// Selection rectangle as it looked like when we just started resizing
        initial_rect: Rectangle,
        /// A key to obtain `&mut Selection` from `Option<Selection>` with a guarantee that it will
        /// always be there (to bypass the limitation that we cannot pass `&mut Selection` in a `Message`)
        sel_is_some: SelectionIsSome,
    },
    /// When we have not yet released the left mouse button
    /// and are dragging the selection to extend it
    ExtendNewSelection(Point),
    /// Left mouse is held down and dragged
    ///
    /// Contains the new point of the mouse
    MovingSelection {
        /// Current position of the cursor
        current_cursor_pos: Point,
        /// Position of the cursor when we first started moving the selection
        initial_cursor_pos: Point,
        /// Current selection
        current_selection: Selection,
        /// Top-left corner of the selection before we started moving it
        initial_rect_pos: Point,
    },
    /// Holding right-click, the selection will move the
    /// nearest corner to the cursor
    ResizingToCursor {
        /// Current position of the cursor
        cursor_pos: Point,
        /// Current selection
        selection: Selection,
        /// A key to obtain `&mut Selection` from `Option<Selection>` with a guarantee that it will
        /// always be there (to bypass the limitation that we cannot pass `&mut Selection` in a `Message`)
        sel_is_some: SelectionIsSome,
    },
    /// Set the selection to the entire screen
    FullSelection,
}
