//! Show errors when something is wrong

use crate::messages::Message;

use std::{
    borrow::Cow,
    time::{Duration, Instant},
};
use iced::{Background, Element, widget::{self, Column, Space, container, row}, Color};
use iced::widget::space;

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
const ERROR_WIDTH: f32 = 300.0;

// When the error appears, how long should it take until it will disappear
const ERROR_DURATION: Duration = Duration::from_secs(3);

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
    pub fn view<'app>(&self, app: &'app super::App) -> Element<'app, Message> {
        let errors = self
            .errors
            .iter()
            .rev()
            // don't display more than the most recent 3 errors
            .take(3)
            .filter(|&error| (error.timestamp.elapsed() < ERROR_DURATION))
            .map(|error| {
                container(widget::text!("Error: {}", error.message))
                    .height(80)
                    .width(ERROR_WIDTH)
                    // .style(|_| container::Style {
                    //     text_color: Some(Color::WHITE),
                    //     background: Some(Background::Color(app.config.theme.error_bg)),
                    //     border: iced::Border {
                    //         color: app.config.theme.drop_shadow,
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

        row![Space::new().width(ERROR_WIDTH), errors].into()
    }
}