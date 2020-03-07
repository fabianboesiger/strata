use iced::{
    Application,
    Command,
    Element,
    Column,
    Text,
    button,
    button::Button,
    text_input,
    text_input::TextInput,
    Background,
    Color
};
use std::{
    fmt,
    path::{
        Path,
        PathBuf
    }
};

#[derive(Debug, Clone)]
pub enum Message {
    PathChanged(String),
    Run
}

enum State {
    Setup,
    Running
}

enum PathOptions {
    Ok(PathBuf),
    NoDir,
    NotFound
}

impl Default for PathOptions {
    fn default() -> PathOptions {
        PathOptions::NotFound
    }
}

impl fmt::Display for PathOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PathOptions::Ok(_) => "Directory found",
            PathOptions::NoDir => "Path is not a directory",
            PathOptions::NotFound => "Invalid path"
        })
    }
}

impl Default for State {
    fn default() -> State {
        State::Setup
    }
}

#[derive(Default)]
pub struct App {
    state: State,
    button_run: button::State,
    text_input_path: text_input::State,
    path_string: String,
    path: PathOptions
}

impl Application for App {
    type Message = Message;

    fn new() -> (App, Command<Message>) {
        let app = App::default();

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Strata")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PathChanged(path) => {
                self.path = if let Ok(path) = Path::new(&path).canonicalize() {
                    if path.as_path().is_dir() {
                        PathOptions::Ok(path)
                    } else {
                        PathOptions::NoDir
                    }
                } else {
                    PathOptions::NotFound
                };
                self.path_string = path;
            },
            Message::Run => {
                self.state = State::Running;
            }
        }
        
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.state {
            State::Setup => {
                Column::new()
                    .spacing(32)
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("Select the directory containing the images").size(24)
                            )
                            .push(
                                TextInput::new(
                                    &mut self.text_input_path,
                                    "Type the directory path here",
                                    &self.path_string,
                                    Message::PathChanged
                                )
                            )
                            .push(
                                Text::new(format!("{}", self.path)).size(16)
                            )
                    )
                    .push(
                        if let PathOptions::Ok(_) = self.path {
                            Button::new(&mut self.button_run, Text::new("Run"))
                                .on_press(Message::Run)
                                .padding(16)
                                .background(Background::Color(Color::from([0.0, 1.0, 0.0])))
                        } else {
                            Button::new(&mut self.button_run, Text::new("Run"))
                                .padding(16)
                                .background(Background::Color(Color::from([1.0, 0.0, 0.0])))
                        }
                    )
                    .into()
            },

            State::Running => {
                Column::new()
                    .push(
                        Text::new(format!("Running ...")).size(24)
                    )
                    .into()
            }
        }
        
    }
}