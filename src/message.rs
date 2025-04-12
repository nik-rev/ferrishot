//! A message represents some event in the app that mutates the state

use iced::{Point, Rectangle, mouse::Cursor};
use rfd::AsyncFileDialog;

use crate::{corners::Side, selection::Selection};

/// Represents an action happening in the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Do nothing
    Noop,
    /// Exits the application
    Exit,
    /// The left mouse button is down
    LeftMouseDown(Cursor),
    /// The left mouse button is up
    LeftMouseUp,
    /// Copy the screenshot to the clipboard
    CopyToClipboard,
    /// Save the screenshot as an image
    SaveScreenshot,
    /// It's a little bit awkward, but this Message is actually only ever sent from part of
    /// `SaveScreenshot`
    SaveScreenshotStep2(AsyncFileDialog, iced::widget::image::Handle, Selection),
    /// The selection is initially resized as it was created
    InitialResize {
        /// Current position of the cursor
        current_cursor_pos: Point,
        /// Initial position of the cursor
        initial_cursor_pos: Point,
        /// Which side we are currently resizing
        resize_side: Side,
        /// Selection rectangle as it looked like when we just started resizing
        initial_rect: Rectangle,
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
}
