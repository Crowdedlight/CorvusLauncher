use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use iced::{Length, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{slider, space, Button, Stack};
use iced::widget::{button, column, container, progress_bar, row, scrollable, text, Space};
use iced::widget::space::{horizontal, vertical};
use crate::{Cli, ServerModList};
use crate::messages::Message;
// use crate::Config;

use crate::ui;
use crate::ui::number_input::NumberInput;
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

    /// vector of our selection listboxes. Stored like this to allow delegation of modpack selections while reusing the component
    pub selection_listboxes: Vec<SelectionListbox>,

    /// Number of HCs to launch
    pub hc_launch_num: NumberInput,
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
            hc_launch_num: NumberInput::default(),
            // config,
            cli,
            // popup: None,
            // TODO, delete default testing
            selection_listboxes: vec![
                SelectionListbox::new(0, "Modpacks".parse().unwrap(), vec![
                    ServerModList::new("Modern".parse().unwrap(), PathBuf::new(), false),
                    ServerModList::new("Cold War".parse().unwrap(), PathBuf::new(), false),
                    ServerModList::new("WW2".parse().unwrap(), PathBuf::new(), false),
                    ServerModList::new("Scifi".parse().unwrap(), PathBuf::new(), false),
                ]),
                SelectionListbox::new(1, "Clientside".parse().unwrap(), vec![
                    ServerModList::new("Clientside Normal".parse().unwrap(), PathBuf::new(), false),
                    ServerModList::new("Clientside Big Event".parse().unwrap(), PathBuf::new(), false),
                ]),
                SelectionListbox::new(2, "Server mods".parse().unwrap(), vec![
                    ServerModList::new("OCAP2".parse().unwrap(), PathBuf::new(), false),
                    ServerModList::new("Advanced Slingloading".parse().unwrap(), PathBuf::new(), false),
                ]),
            ],
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
                    vertical().height(20),
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
                    ),
                    row![
                        button("LAUNCH SERVER").padding(10),
                        horizontal().width(20),
                        column![
                            text("HCs Amount").size(24),
                            self.hc_launch_num.view(self).map(move |message| Message::HcInputChanged(message)),
                            button("LAUNCH HCs").padding(10),
                        ]
                            .align_x(Horizontal::Center)
                            .spacing(4)
                    ]
                        .align_y(Vertical::Bottom)
                        .spacing(10)
                        .padding(15)
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
            },
            Message::HcInputChanged(msg) => {
                self.hc_launch_num.update(msg);
            }
        }

        Task::none()
    }
}