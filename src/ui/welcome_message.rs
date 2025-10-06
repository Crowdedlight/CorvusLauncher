//! The welcome message shown if no config has default values

use std::fmt::Alignment;
use std::sync::Arc;
use iced::{Background, Element, Font, Length::{self, Fill}, alignment::Vertical, widget::{Column, Space, column, row, text, text::Shaping}, widget::space::{horizontal, vertical}, Task, application, Theme, Color};
use iced::alignment::Horizontal;
use iced::widget::{button, space};
use crate::Config;
use crate::configs::config::LocationPaths;
use crate::messages::Message;
use crate::messages::Message::ConfigOpenFileDialog;


// TODO make this stateful instead, I think that will fix a bunch of annoying cases and allow us to
//  store config in struct that it should be able to read from in view...

/// Renders the welcome message
pub fn welcome_message(app: &super::App) -> Element<Message> {

    let file_dialogs = iced::widget::container(
        iced::widget::container(
            column![
                text("Please Configure App").size(30),
                vertical().height(20.0),
                text("Arma3 Server Root").size(20),
                row![
                    // text(app.config.read().unwrap().a3_root.to_string_lossy()).size(20),
                    button("Set Path").on_press(ConfigOpenFileDialog(LocationPaths::A3Root))
                ],
                text("Folder for Modlists").size(20),
                row![
                    // text(app.config.read().unwrap().folder_modlists.to_string_lossy()).size(20),
                    button("Set Path").on_press(ConfigOpenFileDialog(LocationPaths::Modlists))
                ],
                text("Folder for Clientsides").size(20),
                row![
                    // text(app.config.read().unwrap().folder_clientside.to_string_lossy()).size(20),
                    button("Set Path").on_press(ConfigOpenFileDialog(LocationPaths::Clientsides))
                ],
                text("Folder for Server Mods").size(20),
                row![
                    // text(app.config.read().unwrap().folder_servermods.to_string_lossy()).size(20),
                    button("Set Path").on_press(ConfigOpenFileDialog(LocationPaths::ServerMods))
                ],
                vertical().height(15.0),
                row![
                    horizontal().width(450),
                    button("Close").padding(10),
                ]
            ].align_x(Horizontal::Center) // TODO might want this to be left, and use space to push in
        )
            .center_y(Length::Fixed(380.0))
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
            })
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

    column![
        vertical().height(10.0),
        file_dialogs
    ].into()
}

