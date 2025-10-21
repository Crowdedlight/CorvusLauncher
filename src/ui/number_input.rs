use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget::{button, row, text};
use iced::{Element, Length, Task};

#[derive(Debug, Default)]
pub struct NumberInput {
    pub value: u64,
}

#[derive(Clone, Debug)]
pub enum Message {
    ValueIncreased(),
    ValueDecreased(),
}

impl NumberInput {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ValueDecreased() => {
                self.value -= 1;
                Task::none()
            }
            Message::ValueIncreased() => {
                self.value += 1;
                Task::none()
            }
        }
    }
    pub fn view<'app>(&self, _app: &'app super::App) -> Element<'app, Message> {
        row![
            button("-").on_press(Message::ValueDecreased()).padding(5),
            text(self.value.to_string())
                .width(20)
                .size(20)
                .align_y(Vertical::Center)
                .align_x(Horizontal::Center),
            button("+").on_press(Message::ValueIncreased()).padding(5)
        ]
        .into()
    }
}
