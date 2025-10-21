//! The welcome message shown if no config has default values

use crate::Config;
use crate::configs::config::LocationPaths;
use crate::ui::welcome_message::Message::{ConfigOpenFileDialog, SaveUpdateConfig};
use iced::alignment::Horizontal;
use iced::widget::text::Wrapping;
use iced::widget::{button, container, space};
use iced::{
    Background, Color, Element,
    Length::{self},
    Task,
    alignment::Vertical,
    widget::space::{vertical},
    widget::{Space, column, row, text},
};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct WelcomeView {
    a3_root: PathBuf,
    modlists: PathBuf,
    clientsides: PathBuf,
    servermods: PathBuf,
    config: Arc<RwLock<Config>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// open file dialog
    ConfigOpenFileDialog(LocationPaths),
    /// save new config values
    SaveUpdateConfig(PathBuf, PathBuf, PathBuf, PathBuf),
    /// reload views depending on config values
    ReloadViews(),
    /// Error message for catching in main app
    Error(String),
}

impl WelcomeView {
    pub fn new(config: Arc<RwLock<Config>>) -> WelcomeView {
        // load initial values from config
        let c = config.read().unwrap();
        Self {
            a3_root: c.a3_root.clone(),
            modlists: c.folder_modlists.clone(),
            clientsides: c.folder_clientside.clone(),
            servermods: c.folder_servermods.clone(),
            config: config.clone(),
        }
    }
    pub fn view<'app>(&'app self, _app: &'app super::App) -> Element<'app, Message> {
        let file_dialogs = iced::widget::container(
            iced::widget::container(column![
                text("Please Configure App")
                    .size(30)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
                vertical().height(15.0),
                path_selector_element_title("Arma3 Server Root"),
                path_selector_element(
                    self.a3_root.to_string_lossy().parse().unwrap(),
                    LocationPaths::A3Root
                ),
                vertical().height(10.0),
                path_selector_element_title("Folder for Modlists"),
                path_selector_element(
                    self.modlists.to_string_lossy().parse().unwrap(),
                    LocationPaths::Modlists
                ),
                vertical().height(10.0),
                path_selector_element_title("Folder for Clientsides"),
                path_selector_element(
                    self.clientsides.to_string_lossy().parse().unwrap(),
                    LocationPaths::Clientsides
                ),
                vertical().height(10.0),
                path_selector_element_title("Folder for Servermods"),
                path_selector_element(
                    self.servermods.to_string_lossy().parse().unwrap(),
                    LocationPaths::ServerMods
                ),
                vertical().height(15.0),
                container(button("Save").on_press(SaveUpdateConfig(
                    self.a3_root.clone(),
                    self.modlists.clone(),
                    self.clientsides.clone(),
                    self.servermods.clone()
                )))
                .center_x(Length::Fill)
            ])
            .center_y(Length::Fixed(450.0))
            .center_x(Length::Fixed(620.0))
            .style(|_| iced::widget::container::Style {
                text_color: None,
                background: Some(Background::Color(Color::from_rgba8(114, 119, 130, 1.0))),
                border: iced::Border::default()
                    // .color(app.config.theme.info_box_border)
                    .rounded(6.0)
                    .width(1.5),
                shadow: iced::Shadow::default(),
                snap: false,
            }),
        )
        .center(Length::Fill)
        .style(|_| iced::widget::container::Style {
            // text_color: Some(app.config.theme.info_box_fg),
            background: Some(Background::Color(Color::from_rgba8(43, 45, 49, 0.4))),
            text_color: None,
            border: iced::Border::default()
                // .color(app.config.theme.info_box_border)
                .rounded(6.0)
                .width(1.5),
            shadow: iced::Shadow::default(),
            snap: false,
        });

        column![vertical().height(10.0), file_dialogs].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConfigOpenFileDialog(location_type) => {
                // Run function to open filedialog and save return
                let path = open_file_dialog();
                log::debug!(
                    "update_paths: type: {:?}, handle: {:?}",
                    location_type,
                    path
                );

                match location_type {
                    LocationPaths::A3Root => self.a3_root = path,
                    LocationPaths::Modlists => self.modlists = path,
                    LocationPaths::Clientsides => self.clientsides = path,
                    LocationPaths::ServerMods => self.servermods = path,
                };
                Task::none()
            }
            Message::SaveUpdateConfig(a3root, modlist, clientsides, servermods) => {
                let result_config = self.config.write().unwrap().update_config(
                    a3root,
                    modlist,
                    clientsides,
                    servermods,
                );

                // if error, return error
                if let Err(err) = result_config {
                    return Task::done(Message::Error(err.to_string()));
                }

                // TODO the right way here would be to have this class depend on reference to config and its paths, as then change of paths would trigger a reload of files
                //  but if this works for now, we leave it be, as the other is a bigger change
                Task::done(Message::ReloadViews())
            }
            Message::ReloadViews() => {
                //this is only called to allow to send a reloadViews message that can get captured by super. So here we do noting
                log::debug!("ReloadViews() called");
                Task::none()
            }
            Message::Error(_) => {
                //this is only called to allow to send a error message that can get captured by super. So here we do noting
                log::debug!("Welcome_message Error() called");
                Task::none()
            }
        }
    }
}

/// function to open file dialog to pick folder. We don't need async as nothing else in app has to run while picking
pub fn open_file_dialog() -> PathBuf {
    // if we run into issues with slow saving to disk, either change to save all values at once, or make it async, but that will open new problems...

    // open filedialog
    let selection = rfd::FileDialog::new().pick_folder();

    match selection {
        Some(a3) => a3,
        None => PathBuf::new(),
    }

    // if let Some(selection) = selection {
    //     Ok(selection)
    // } else {
    //     Err(anyhow::anyhow!(""))
    // }
}

pub fn path_selector_element_title(title: &str) -> Element<'static, Message> {
    row![space().width(10.0), text(title.to_string()).size(20)].into()
}

pub fn path_selector_element(
    path: String,
    location_type: LocationPaths,
) -> Element<'static, Message> {
    row![
        Space::new().width(Length::Fixed(10.0)),
        container(
            text(path)
                .size(16)
                .wrapping(Wrapping::None)
                .align_y(Vertical::Center)
        )
        .align_left(Length::Fill)
        .padding(10)
        .style(|_| container::Style {
            text_color: None,
            background: None,
            border: iced::Border::default().width(1.5).color(Color::BLACK),
            shadow: Default::default(),
            snap: true,
        }),
        container(button("Set Path").on_press(ConfigOpenFileDialog(location_type)))
            .align_right(Length::Shrink)
            .padding(5)
            .align_y(Vertical::Center)
    ]
    .into()
}
