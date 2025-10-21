use std::cmp::PartialEq;
use crate::messages::Message;
use crate::{Cli, Config, ServerModList};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::space::{horizontal, vertical};
use iced::widget::{Button, Stack, rule, slider, space, text_input};
use iced::widget::{Rule, Space, button, column, container, progress_bar, row, scrollable, text};
use iced::{Element, Length, Task};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use crate::arma::server_launch::{launch_hc, launch_server};
use crate::arma::server_modlist::load_modlists;
// use crate::Config;

use super::Errors;
use crate::ui;
use crate::ui::number_input::NumberInput;
use crate::ui::profile_chooser::ProfileChooser;
use crate::ui::selection_listbox::SelectionListbox;
use crate::ui::welcome_message::WelcomeView;

/// Holds the state
#[derive(Debug)]
pub struct App {
    /// When the application was launched
    pub time_started: Instant,
    /// How long has passed since starting
    pub time_elapsed: Duration,
    /// Config of the app
    pub config: Arc<RwLock<Config>>,
    /// Errors to display to the user
    pub errors: Errors,
    /// Command line arguments passed
    pub cli: Arc<Cli>,
    /// port number for server and HC
    pub port_num: String,

    // Currently opened popup
    // pub popup: Option<Popup>,
    /// vector of our selection listboxes. Stored like this to allow delegation of modpack selections while reusing the component
    pub selection_listboxes: Vec<SelectionListbox>,

    /// Number of HCs to launch
    pub hc_launch_num: NumberInput,

    /// Server profile chooser
    pub server_profile_chooser: ProfileChooser,

    /// welcome message - aka set config
    pub welcome_view: WelcomeView,
}


#[bon::bon]
impl App {
    #[builder]
    pub fn new(cli: Arc<Cli>, configs: Arc<RwLock<Config>>) -> Self {

        // make empty vectors
        let mut modpacks: Vec<ServerModList> = vec![];
        let mut clientside: Vec<ServerModList> = vec![];
        let mut servermod: Vec<ServerModList> = vec![];

        // try and load modlists from folders if config is valid
        let c = configs.clone();
        if c.read().unwrap().is_config_valid() {
            // modpacks
            modpacks = load_modlists(&c.read().unwrap().folder_modlists);
            // clientsides
            clientside = load_modlists(&c.read().unwrap().folder_clientside);
            // servermods
            servermod = load_modlists(&c.read().unwrap().folder_servermods);
        }

        Self {
            time_started: Instant::now(),
            time_elapsed: Duration::ZERO,
            errors: Errors::default(),
            hc_launch_num: NumberInput::default(),
            server_profile_chooser: ProfileChooser::new(
                configs.clone().read().unwrap().server_profiles.clone(),
            ),
            welcome_view: WelcomeView::new(configs.clone()),
            config: configs,
            cli,
            port_num: "2302".to_string(),
            // popup: None,
            selection_listboxes: vec![
                SelectionListbox::new(
                    0,
                    "Modpacks".parse().unwrap(),
                    modpacks,
                ),
                SelectionListbox::new(
                    1,
                    "Clientside".parse().unwrap(),
                    clientside,
                ),
                SelectionListbox::new(
                    2,
                    "Server mods".parse().unwrap(),
                    servermod,
                ),
            ],
        }
    }

    /// Renders the app
    pub fn view(&self) -> iced::Element<Message> {
        // conditionally set to Some or None to show this view
        let mut welcome_view: Option<Element<Message>> = None;
        // only push welcome for configuration if current config is invalid
        if !self.config.read().unwrap().is_config_valid() {
            welcome_view = Some(
                self.welcome_view
                    .view(self)
                    .map(move |msg| Message::WelcomeViewMessage(msg)),
            );
        }

        Stack::new()
            // Main window
            .push(column![
                // titel
                text("CorvusLauncher")
                    .width(Length::Fill)
                    .size(40)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Top),
                vertical().height(20),
                // listboxes
                row(self
                    .selection_listboxes
                    .iter()
                    .map(SelectionListbox::view)
                    .enumerate()
                    .map(|(index, listbox)| {
                        // Here we turn our `Element<counter::Message>` into
                        // an `Element<Message>` by combining the `index` and the
                        // message of the `element`.
                        listbox.map(move |message| Message::SelectionBoxUpdate(index, message))
                    }),),
                rule::horizontal(2),
                row![
                    self.server_profile_chooser
                        .view(self)
                        .map(move |msg| Message::ServerProfileChanged(msg)),
                    horizontal().width(20),
                    column![
                        text("Port").size(24),
                        text_input("", &*self.port_num).on_input(Message::ChangePortNumber)
                            .width(60)
                            // .size(20)
                            .align_x(Horizontal::Center),
                        button("LAUNCH SERVER").padding(10).on_press(Message::LaunchServer()),
                    ]
                        .align_x(Horizontal::Center)
                        .spacing(4),
                    horizontal().width(20),
                    column![
                        text("HCs Amount").size(24),
                        self.hc_launch_num
                            .view(self)
                            .map(move |message| Message::HcInputChanged(message)),
                        button("LAUNCH HCs").padding(10).on_press(Message::LaunchHCs()),
                    ]
                    .align_x(Horizontal::Center)
                    .spacing(4)
                ]
                .align_y(Vertical::Center)
                .spacing(100)
                .padding(15)
                .width(Length::Fill)
            ])
            .push(welcome_view)
            .push(self.errors.view(self, Message::ClearErrors))
            .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {

        match message {
            // Message::ClosePopup => {
            //     self.popup = None;
            // }
            Message::NoOp => (),
            Message::Error(err) => {
                self.errors.push(err);
            }
            Message::SelectionBoxUpdate(index, listbox_msg) => {
                if let Some(listbox) = self.selection_listboxes.get_mut(index) {
                    listbox.update(listbox_msg);
                }
            }
            Message::HcInputChanged(msg) => {
                return self.hc_launch_num.update(msg).map(Message::HcInputChanged);
            }
            Message::ServerProfileChanged(msg) => {
                return self.server_profile_chooser.update(msg).map(Message::ServerProfileChanged);
            }
            Message::WelcomeViewMessage(msg) => {

                // handle specific reload message that has to run in parent view
                if msg == ui::welcome_message::Message::ReloadViews() {
                    // Reload views depending on config values, such as the listboxes
                    let c = self.config.clone();
                    let modlist = load_modlists(&c.read().unwrap().folder_modlists);
                    let clientside = load_modlists(&c.read().unwrap().folder_clientside);
                    let servermods = load_modlists(&c.read().unwrap().folder_servermods);

                    self.selection_listboxes.get_mut(0).unwrap().elements = modlist;
                    self.selection_listboxes.get_mut(1).unwrap().elements = clientside;
                    self.selection_listboxes.get_mut(2).unwrap().elements = servermods;
                }

                // pass message on, has to return here as otherwise we would never get messages initiated in WelcomeViewMessage update()
                return self.welcome_view.update(msg).map(Message::WelcomeViewMessage);
            }
            Message::ChangePortNumber(new_port) => {
                self.port_num = new_port;
            }
            Message::LaunchServer() => {
                // combine selected mods
                let everyone_mods: Vec<PathBuf> = self.selection_listboxes.get(0).unwrap().elements.iter().filter(|e| e.selected).flat_map(|e| e.mods.clone()).collect();
                let clientside_mods: Vec<PathBuf> = self.selection_listboxes.get(1).unwrap().elements.iter().filter(|e| e.selected).flat_map(|e| e.mods.clone()).collect();
                let server_mods: Vec<PathBuf> = self.selection_listboxes.get(2).unwrap().elements.iter().filter(|e| e.selected).flat_map(|e| e.mods.clone()).collect();

                let c = self.config.clone();

                // launch server
                let launch_result = launch_server(
                    &c.read().unwrap().a3_root,
                    &c.read().unwrap().a3_server_executable,
                    &self.port_num,
                    &self.server_profile_chooser.get_selected_profile(),
                    everyone_mods,
                    clientside_mods,
                    server_mods,
                );
                // handle error
                if let Err(err) = launch_result {
                    return Task::done(Message::Error(err.to_string()))
                }
            }
            Message::LaunchHCs() => {
                // get config
                let c = self.config.clone();

                // launch HCs
                for i in 0 .. self.hc_launch_num.value {
                    let launch_result = launch_hc(
                        &c.read().unwrap().a3_root,
                        &c.read().unwrap().a3_server_executable,
                        &self.port_num,
                        i
                    );

                    // handle error
                    if let Err(err) = launch_result {
                        return Task::done(Message::Error(err.to_string()))
                    }
                }
            }
            Message::ClearErrors() => {
                // clear errors
                self.errors.errors.clear();
            }
        }

        Task::none()
    }
}
