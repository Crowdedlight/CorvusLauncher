//! A message represents some event in the app that mutates the global state

use crate::ui;
use std::time::Instant;

/// Handles all mutation of the global state, the `App`.
pub trait Handler {
    /// Handle the message, mutating the `App`.
    fn handle(self, app: &mut crate::App) -> iced::Task<Message>;
}

/// Represents an action happening in the application
#[derive(Debug, Clone)]
pub enum Message {
    // Close the current popup
    // ClosePopup,
    /// Letters message
    // Letters(ui::popup::letters::Message),
    /// Update Selection listbox, consists of unique name of listbox, index of element in it, and the state
    SelectionBoxUpdate(usize, ui::selection_listbox::Message),
    /// Update HC count
    HcInputChanged(ui::number_input::Message),

    /// An error occured, display to the user
    Error(String),
    /// Do nothing
    NoOp,
}