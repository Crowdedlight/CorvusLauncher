//! A message represents some event in the app that mutates the global state
use crate::ui;

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
    /// Update server profile selected
    ServerProfileChanged(ui::profile_chooser::Message),
    /// welcome view messages
    WelcomeViewMessage(ui::welcome_message::Message),
    /// port number change message
    ChangePortNumber(String),
    /// Launch server
    LaunchServer(),
    /// Launch HCs
    LaunchHCs(),

    /// An error occured, display to the user
    Error(String),
    /// clear Error messages
    ClearErrors(),
    /// Do nothing
    NoOp,
}
