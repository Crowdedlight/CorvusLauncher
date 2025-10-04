use std::path::PathBuf;
use iced::{Element, Task};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::pick_list;
use iced::widget::{button, column, text};

#[derive(Debug, Default)]
pub struct ProfileChooser {
    selected: Option<String>,
    options: Vec<String>
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    ProfileSelected(String),
}

impl ProfileChooser {

    pub fn new() -> ProfileChooser {

        // TODO get list of all profile options from A3 master dir
        // TODO we might just hardcode instead, to avoid the ownership challenge and be able to make new profile?
        //  (We could store options in config, and allow to add new profiles from UI)

        Self {selected : Some("ServerNormal".parse().unwrap()), options : vec!["ServerLib".parse().unwrap(), "ServerATF".parse().unwrap(), "ServerEvent".parse().unwrap(), "ServerNormal".parse().unwrap()]} // TODO debugging
    }
    pub(crate) fn view<'app>(&self, app: &'app super::App) -> Element<'app, Message> {

        column![
            text("Server Profile").size(24),
            pick_list(self.options.clone(), self.selected.clone(), Message::ProfileSelected)
        ]
            .align_x(Horizontal::Center)
            .spacing(4)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProfileSelected(selection) => {
                self.selected = Some(selection);
                Task::none()
            },
        }
    }
}