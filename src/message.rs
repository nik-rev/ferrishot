//! A message represents some event in the app that mutates the state

use crate::ui;

use crate::config::KeyAction;

/// Handler for a `Message`
pub trait Handler {
    /// Handle the message
    fn handle(self, app: &mut crate::App) -> Option<iced::Task<Message>>;
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
    /// Selection message
    Selection(ui::selection::Message),
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
