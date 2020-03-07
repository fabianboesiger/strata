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
    },
    env
};
use crate::operator::run;
use crate::error;

#[derive(Clone, Debug)]
pub enum Message {
    LoadPathChanged(String),
    SavePathChanged(String),
    Run,
    Finish(error::Result<()>)
}

enum State {
    Setup,
    Running,
    Finished(error::Result<()>)
}

enum PathOptions {
    Ok(PathBuf),
    NoDir,
    NotFound
}

impl Default for PathOptions {
    fn default() -> PathOptions {
        PathOptions::Ok(env::current_dir().unwrap())
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
    run_button: button::State,
    restart_button: button::State,
    exit_button: button::State,
    load_path_text_input: text_input::State,
    load_path_string: String,
    load_path: PathOptions,
    save_path_text_input: text_input::State,
    save_path_string: String,
    save_path: PathOptions
}

impl Application for App {
    type Message = Message;

    fn new() -> (App, Command<Message>) {
        let working_dir = env::current_dir().unwrap();
        let mut load_dir = working_dir.clone();
        load_dir.push("images");
        let mut save_dir = working_dir.clone();
        save_dir.push("result.jpg");

        let app = App {
            load_path_string: format!("{}", load_dir.display()),
            load_path: PathOptions::Ok(load_dir),
            save_path_string: format!("{}", save_dir.display()),
            save_path: PathOptions::Ok(save_dir),
            ..
            App::default()
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Strata")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadPathChanged(path) => {
                self.load_path = if let Ok(path) = Path::new(&path).canonicalize() {
                    if path.as_path().is_dir() {
                        PathOptions::Ok(path)
                    } else {
                        PathOptions::NoDir
                    }
                } else {
                    PathOptions::NotFound
                };
                self.load_path_string = path;

                Command::none()
            },
            Message::SavePathChanged(path) => {
                self.save_path = if let Ok(path) = Path::new(&path).canonicalize() {
                    if path.as_path().is_dir() {
                        PathOptions::Ok(path)
                    } else {
                        PathOptions::NoDir
                    }
                } else {
                    PathOptions::NotFound
                };
                self.save_path_string = path;

                Command::none()
            },
            Message::Run => {
                self.state = State::Running;

                Command::perform(run(), Message::Finish)
            },
            Message::Finish(result) => {
                self.state = State::Finished(result);

                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &self.state {
            State::Setup => {
                Column::new()
                    .spacing(32)
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("Input Images")
                                    .size(32)
                            )
                            .push(
                                Text::new("Please select the directory containing your input images.")
                            )
                            .push(
                                TextInput::new(
                                    &mut self.load_path_text_input,
                                    "Type the path here.",
                                    &self.load_path_string,
                                    Message::LoadPathChanged
                                )
                                    .padding(8)
                            )
                            .push(
                                Text::new(format!("{}", match self.load_path {
                                    PathOptions::Ok(_) => "Directory found",
                                    PathOptions::NoDir => "Path is not a directory",
                                    PathOptions::NotFound => "Invalid path"
                                }))
                            )
                    )
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("Output Image").size(32)
                            )
                            .push(
                                Text::new("Please select a path for your output image.")
                            )
                            .push(
                                TextInput::new(
                                    &mut self.save_path_text_input,
                                    "Type the path here.",
                                    &self.save_path_string,
                                    Message::SavePathChanged
                                )
                                    .padding(8)
                            )
                            .push(
                                Text::new(format!("{}", match self.load_path {
                                    PathOptions::Ok(_) => "Directory found",
                                    PathOptions::NoDir => "Path is not a directory",
                                    PathOptions::NotFound => "Invalid path"
                                }))
                            )
                    )
                    .push(
                        if let PathOptions::Ok(_) = self.load_path {
                            Button::new(&mut self.run_button, Text::new("Run"))
                                .on_press(Message::Run)
                                .padding(16)
                                //.background(Background::Color(Color::from([0.0, 2.0, 0.0])))
                        } else {
                            Button::new(&mut self.run_button, Text::new("Run"))
                                .padding(16)
                                //.background(Background::Color(Color::from([2.0, 0.0, 0.0])))
                        }
                    )
                    .into()
            },
            State::Running => {
                Column::new()
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("Processing ...")
                                    .size(32)
                            )
                            .push(
                                Text::new("The computation of your image can take a while depending on your hardware.")
                            )
                    )
                    .into()
            },
            State::Finished(Ok(())) => {
                Column::new()
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("Your Image is Ready")
                                    .size(32)
                            )
                            .push(
                                Text::new(format!("The resulting image was saved under \"{}\".", self.save_path_string))
                            )
                    )
                    .into()
            },
            State::Finished(Err(error)) => {
                Column::new()
                    .push(
                        Column::new()
                            .spacing(8)
                            .push(
                                Text::new("An Error Occured")
                                    .size(32)
                            )
                            .push(
                                Text::new(format!("{}", error))
                            )
                    )
                    .into()
            }
        }
    }
}