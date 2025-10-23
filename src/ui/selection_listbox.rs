//! A listbox consisting of title, and a scrollable box with available presets. Reused across All_mods, Server_mods and Clientside mods
use crate::ServerModList;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, row, scrollable, text};
use iced::{Element, Length, Task, Theme};

#[derive(Debug)]
pub struct SelectionListbox {
    pub id: usize,
    pub title: String,
    pub elements: Vec<ServerModList>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ToggleSelection(usize, bool),
}

impl SelectionListbox {
    pub fn new(id: usize, title: String, elements: Vec<ServerModList>) -> Self {
        Self {
            id,
            title,
            elements,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleSelection(i, checked) => {
                if let Some(element) = self.elements.get_mut(i) {
                    element.selected = checked;
                }
                Task::none()
            }
        }
    }

    // /// helper function to handle toggle event on checkbox, identical behavior as the button on_press
    // pub fn checkbox_toggled(id: u64, checked: bool) -> Message::ToggleSelection {
    //     Message::ToggleSelection(id as usize, checked);
    // }

    pub fn view(&self) -> Element<'_, Message> {
        // make list of selections
        let selection_list =
            self.elements
                .iter()
                .enumerate()
                .fold(column![].spacing(6), |col, (i, modlist)| {
                    col.push(row![
                        button(row![
                            text(&modlist.name)
                                .width(Length::FillPortion(6)),
                            text(format!("[{}]", &modlist.mods.len()))
                                .width(Length::FillPortion(2)),
                            checkbox("", modlist.selected),
                        ])
                        .padding(8)
                        .style(|theme: &Theme, status| {
                            let palette = theme.extended_palette();
                            match modlist.selected {
                                false => button::Style::default()
                                    .with_background(palette.secondary.base.color),
                                _ => button::primary(theme, status),
                            }
                        })
                        .width(Length::Fill)
                        .on_press(Message::ToggleSelection(i, !modlist.selected)),
                        // Space::with_width(15)
                    ])
                });

        let scrollable: Element<Message> = scrollable(selection_list)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        column![
            text(&self.title)
                .size(30)
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Top),
            scrollable
        ]
        .padding(10)
        .into()
    }
}
