use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use iced::{Length, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Stack};
use iced::widget::{button, column, container, progress_bar, row, scrollable, text, Space};
use crate::Cli;
use crate::messages::Message;
// use crate::Config;

use crate::ui;
use crate::ui::selection_listbox::SelectionListbox;
use super::Errors;


/// Holds the state
#[derive(Debug)]
pub struct App {
    /// If an image is in the process of being uploaded (but hasn't yet)
    pub is_uploading_image: bool,
    /// When the application was launched
    pub time_started: Instant,
    /// How long has passed since starting ferrishot
    pub time_elapsed: Duration,
    /// Config of the app
    // pub config: Arc<Config>,
    /// Errors to display to the user
    pub errors: Errors,
    /// Command line arguments passed
    pub cli: Arc<Cli>,

    // Currently opened popup
    // pub popup: Option<Popup>,

    /// hashmap of our selection listboxes. Stored like this to allow delegation of modpack selections while reusing the component
    pub selection_listboxes: Vec<SelectionListbox>
}
#[bon::bon]
impl App {
    #[builder]
    pub fn new(
        cli: Arc<Cli>,
        // config: Arc<Config>,
    ) -> Self {
        Self {
            is_uploading_image: false,
            time_started: Instant::now(),
            time_elapsed: Duration::ZERO,
            errors: Errors::default(),
            // config,
            cli,
            // popup: None,
            selection_listboxes: vec![],
        }
    }

    /// Renders the app
    pub fn view(&self) -> iced::Element<Message> {
        Stack::new()
            // Main window
            .push(
                column![
                    // titel
                    text("CorvusLauncher")
                        .width(Length::Fill)
                        .size(40)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Top),
                    // listboxes
                    row(
                        self.selection_listboxes
                            .iter()
                            .map(SelectionListbox::view)
                            .enumerate()
                            .map(|(index, listbox)| {
                                // Here we turn our `Element<counter::Message>` into
                                // an `Element<Message>` by combining the `index` and the
                                // message of the `element`.
                                listbox.map(move |message| Message::SelectionBoxUpdate(index, message))
                            }),
                    )
                    ]
            )
            // information popup with basic tips
            // .push_maybe(
            //     (self.popup.is_none() && self.selection.is_none())
            //         .then(|| super::welcome_message(self)),
            // )
            // errors
            .push(self.errors.view(self))
            .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use crate::messages::Handler as _;

        match message {
            // Message::ClosePopup => {
            //     self.popup = None;
            // }
            Message::NoOp => (),
            // Message::Command { action, count } => {
            //     return <crate::Command as crate::command::Handler>::handle(action, self, count);
            // }
            Message::Error(err) => {
                self.errors.push(err);
            },
            Message::SelectionBoxUpdate(index, listbox_msg) => {
                if let Some(listbox) = self.selection_listboxes.get_mut(index) {
                    listbox.update(listbox_msg);
                }
            }
        }

        Task::none()
    }
}