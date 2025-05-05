//! A message represents some event in the app that mutates the state

use std::time::Instant;

use crate::ui;

use crate::config::KeyAction;

/// Handler for a `Message`
pub trait Handler {
    /// Handle the message
    fn handle(self, app: &mut crate::App) -> iced::Task<Message>;
}

/// Represents an action happening in the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Close the app
    Exit,
    /// Close the current popup
    ClosePopup,
    /// Image uploaded message
    ImageUploaded(ui::popup::image_uploaded::Message),
    /// A certain moment. This message is used for animations
    Tick(Instant),
    /// Letters message
    Letters(ui::popup::letters::Message),
    /// Size indicator message
    SizeIndicator(ui::size_indicator::Message),
    /// Selection message
    Selection(Box<ui::selection::Message>),
    /// Keybinding cheatsheet message
    KeyCheatsheet(ui::popup::keybindings_cheatsheet::Message),
    /// An error occured, display to the user
    Error(String),
    /// Do nothing
    NoOp,
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
