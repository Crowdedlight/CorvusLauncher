//! Show errors when something is wrong

use crate::messages::Message;

use iced::widget::{button, scrollable};
use iced::{
    Background, Color, Element, Length,
    widget::{self, Column, column, container},
};
use std::{borrow::Cow, time::Instant};

/// Show an error message to the user
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct ErrorMessage {
    // Error message
    pub message: Cow<'static, str>,
    // When the error was created
    pub timestamp: Instant,
}

impl ErrorMessage {
    // Create a new error message
    pub fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Self {
            message: message.into(),
            timestamp: Instant::now(),
        }
    }
}

// error message Width
const ERROR_WIDTH: f32 = 600.0;

/// Render errors on the screen
#[derive(Default, Debug)]
pub struct Errors {
    // A list of errors to show
    pub errors: Vec<ErrorMessage>,
}

impl Errors {
    pub fn push<T: Into<Cow<'static, str>> + std::fmt::Display>(&mut self, error: T) {
        self.errors.push(ErrorMessage::new(error));
    }

    // Show errors on the screen
    pub fn view<'app>(
        &self,
        _app: &'app super::App,
        on_clear: impl Fn() -> Message,
    ) -> Option<Element<'app, Message>> {
        // if no errors, we just return
        if self.errors.is_empty() {
            return None;
        }

        let errors = self
            .errors
            .iter()
            .rev()
            .map(|error| {
                container(widget::text!("Errors:\n{}", error.message))
                    .height(80)
                    .width(ERROR_WIDTH)
                    // .style(|_| container::Style {
                    //     text_color: Some(Color::WHITE),
                    //     background: Some(Background::Color(app.configs.theme.error_bg)),
                    //     border: iced::Border {
                    //         color: app.configs.theme.drop_shadow,
                    //         width: 4.0,
                    //         radius: 2.0.into(),
                    //     },
                    //     shadow: iced::Shadow::default(),
                    // })
                    .padding(10.0)
                    .into()
            })
            .collect::<Column<_>>()
            .width(ERROR_WIDTH)
            .spacing(30);

        Some(
            container(
                container(column![
                    scrollable(errors),
                    container(button("Clear Errors").on_press(on_clear()))
                        .align_bottom(Length::Fill)
                        .align_right(Length::Fill)
                        .padding(20)
                ])
                .align_left(Length::Fixed(620.0))
                .align_top(Length::Fixed(450.0))
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
            })
            .into(),
        )
    }
}
