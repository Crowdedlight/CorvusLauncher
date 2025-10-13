use iced::alignment::{Horizontal, Vertical};
use iced::widget::pick_list;
use iced::widget::{button, column, text};
use iced::{Element, Task};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ProfileChooser {
    selected: Option<String>,
    options: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(String),
}

impl ProfileChooser {
    pub fn new(profiles: Vec<String>) -> ProfileChooser {
        // get initial profiles from config
        Self {
            selected: profiles.first().map(|p| p.to_owned().clone()),
            options: profiles,
        }
    }
    pub fn view<'app>(&self, app: &'app super::App) -> Element<'app, Message> {
        // TODO add gui to add new server profiles?

        column![
            text("Server Profile").size(24),
            pick_list(
                self.options.clone(),
                self.selected.clone(),
                Message::ProfileSelected
            ),
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
            }
        }
    }

    pub fn get_selected_profile(&self) -> String {
        self.selected.clone().expect("No selected profile")
    }
}
