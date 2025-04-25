//! A message represents some event in the app that mutates the state

use iced::{Point, Rectangle, mouse::Cursor};

use crate::{
    config::KeyAction,
    rect::SideOrCorner,
    ui::{
        self,
        selection::{Selection, SelectionIsSome, Speed},
    },
};

pub trait Handler {
    fn handle(self, app: &mut crate::App);
}

/// Represents an action happening in the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Image uploaded message
    ImageUploaded(ui::image_uploaded::Message),
    /// Letters message
    Letters(ui::letters::Message),
    /// Size indicator message
    SizeIndicator(ui::size_indicator::Message),
    /// An error occured, display to the user
    Error(String),
    /// Do nothing
    NoOp,
    /// Upload screenshot to the internet
    Upload,
    /// The left mouse button is down
    LeftMouseDown(Cursor),
    /// Enter idle mode
    EnterIdle,
    /// The selection is currently being resized
    Resize {
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
        /// Multiplier for how fast we are resizing.
        speed: Speed,
    },
    /// When we have not yet released the left mouse button
    /// and are dragging the selection to extend it
    ExtendNewSelection(Point),
    /// Left mouse is held down and dragged
    ///
    /// Contains the new point of the mouse
    MoveSelection {
        /// Current position of the cursor
        current_cursor_pos: Point,
        /// Position of the cursor when we first started moving the selection
        initial_cursor_pos: Point,
        /// Current selection
        current_selection: Selection,
        /// Top-left corner of the selection before we started moving it
        initial_rect_pos: Point,
        /// How fast the selection should move
        speed: Speed,
    },
    /// Holding right-click, the selection will move the
    /// nearest corner to the cursor
    ResizeToCursor {
        /// Current position of the cursor
        cursor_pos: Point,
        /// Current selection
        selection: Selection,
        /// A key to obtain `&mut Selection` from `Option<Selection>` with a guarantee that it will
        /// always be there (to bypass the limitation that we cannot pass `&mut Selection` in a `Message`)
        sel_is_some: SelectionIsSome,
    },
    /// An action can be triggered by a keybind
    ///
    /// It can also be triggered through other means, such as pressing a button
    KeyBind {
        /// What to do when this keybind is pressed
        action: KeyAction,
        /// How many times it was pressed
        ///
        /// This does not always have an effect, such as it does not make sense to
        /// move the selection to the center several times
        ///
        /// It has an effect for stuff like moving the selection right by `N` pixels
        /// in which case we'd move to the right by `N * count` instead
        count: u32,
    },
}
