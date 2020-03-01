use iced::{
    Application,
    Command,
    Element,
    Column,
    Text,
    Radio,
    button,
    button::Button
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Load,
    Position
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OperationType::Load => "Load images",
            OperationType::Position => "Find relative positions of the layers"
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OperationSelected(OperationType),
    ApplyOperation(OperationType),
    ReverseOperation
}

enum State {
    SelectOperation(Option<OperationType>),
    OperationOptions(OperationType)
}

pub struct App {
    state: State,
    button_next: button::State
}

impl Application for App {
    type Message = Message;

    fn new() -> (App, Command<Message>) {
        let app = App {
            state: State::SelectOperation(None),
            button_next: button::State::new()
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Strata")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OperationSelected(operation) => {
                self.state = State::SelectOperation(Some(operation))
            },
            Message::ApplyOperation(operation) => {
                self.state = State::OperationOptions(operation)
            },
            Message::ReverseOperation => {

            }
        }
        
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.state {
            State::SelectOperation(operation) => {
                Column::new()
                    .push(
                        Text::new("What operation would you like to perform next?").size(48)
                    )
                    .push(
                        Radio::new(OperationType::Load, "Load images from directory", operation, Message::OperationSelected)
                    )
                    .push(
                        Radio::new(OperationType::Position, "Find position of layers relative to each other", operation, Message::OperationSelected)
                    )
                    .push(
                        if let Some(operation) = operation {
                            Button::new(&mut self.button_next, Text::new("Apply operation"))
                                .on_press(Message::ApplyOperation(operation))
                        } else {
                            Button::new(&mut self.button_next, Text::new("Apply operation"))
                        }
                    )
                    .into()
            }
            State::OperationOptions(operation) => {
                Column::new()
                    .push(
                        Text::new(format!("Set options for {}", operation)).size(48)
                    )
                    .into()
            }
        }
        
    }
}