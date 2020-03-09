use iced::{
    Application,
    Command,
    Element,
    Column,
    Row,
    Text,
    button,
    button::Button,
    text_input,
    text_input::TextInput,
    Background,
    Color,
    Scrollable,
    scrollable
};
use std::{
    path::PathBuf,
    env,
    process::exit
};
use crate::operator::run;
use crate::error;

#[derive(Clone, Debug)]
pub enum Message {
    LoadPathChanged(String),
    SavePathChanged(String),
    Run,
    Finish(error::Result<()>),
    Restart,
    Exit
}

enum State {
    Setup,
    Running,
    Finished(error::Result<()>)
}

#[derive(PartialEq)]
enum LoadPathOptions {
    Ok,
    NotFound,
    NoDir
}

impl LoadPathOptions {
    fn check(path: &PathBuf) -> LoadPathOptions {
        if path.is_dir() {
            LoadPathOptions::Ok
        } else
        if path.exists() {
            LoadPathOptions::NoDir
        } else {
            LoadPathOptions::NotFound
        }
    }
}

#[derive(PartialEq)]
enum SavePathOptions {
    Ok,
    InvalidExtension,
    NotFound,
    AlreadyExists
}

impl SavePathOptions {
    fn check(path: &PathBuf) -> SavePathOptions {
        if path.is_file() {
            SavePathOptions::AlreadyExists
        } else
        if let Some(parent) = path.parent() {
            if parent.is_dir() {
                if let Some(extension) = path.extension() {
                    let extension = extension.to_string_lossy();
                    if extension == "jpg" {
                        SavePathOptions::Ok
                    } else {
                        SavePathOptions::InvalidExtension
                    }
                } else {
                    SavePathOptions::InvalidExtension
                }
            } else {
                SavePathOptions::NotFound
            }
        } else {
            SavePathOptions::NotFound
        }
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
    load_path: PathBuf,
    save_path_text_input: text_input::State,
    save_path: PathBuf,
    scroll: scrollable::State
}

impl Application for App {
    type Message = Message;

    fn new() -> (App, Command<Message>) {
        let working_dir = env::current_dir().unwrap();
        let mut load_path = working_dir.clone();
        load_path.push("images");
        let mut save_path = working_dir.clone();
        save_path.push("result.jpg");

        let app = App {
            load_path,
            save_path,
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
                /*self.load_path = if let Ok(path) = Path::new(&path).canonicalize() {
                    if path.as_path().is_dir() {
                        PathOptions::Ok(path)
                    } else {
                        PathOptions::NoDir
                    }
                } else {
                    PathOptions::NotFound
                };*/
                self.load_path = PathBuf::from(path);

                Command::none()
            },
            Message::SavePathChanged(path) => {
                /*self.save_path = if let Ok(path) = Path::new(&path).canonicalize() {
                    if path.as_path().is_dir() {
                        PathOptions::Ok(path)
                    } else {
                        PathOptions::NoDir
                    }
                } else {
                    PathOptions::NotFound
                };*/
                self.save_path = PathBuf::from(path);

                Command::none()
            },
            Message::Run => {
                self.state = State::Running;

                Command::perform(run(self.load_path.clone(), self.save_path.clone()), Message::Finish)
            },
            Message::Finish(result) => {
                self.state = State::Finished(result);

                Command::none()
            },
            Message::Restart => {
                self.state = State::Setup;

                Command::none()
            },
            Message::Exit => {
                exit(0);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Scrollable::new(&mut self.scroll)
            .padding(32)
            .push(match &self.state {
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
                                        &self.load_path.to_string_lossy(),
                                        Message::LoadPathChanged
                                    )
                                        .padding(8)
                                )
                                .push(
                                    match LoadPathOptions::check(&self.load_path) {
                                        LoadPathOptions::Ok => 
                                            Text::new("Directory found.")
                                                .color(Color::from([0.3, 0.7, 0.3])),
                                        LoadPathOptions::NotFound => 
                                            Text::new("Invalid Path.")
                                                .color(Color::from([0.7, 0.3, 0.3])),
                                        LoadPathOptions::NoDir => 
                                            Text::new("Path is not a directory.")
                                                .color(Color::from([0.7, 0.3, 0.3]))
                                    }
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
                                        &self.save_path.to_string_lossy(),
                                        Message::SavePathChanged
                                    )
                                        .padding(8)
                                )
                                .push(
                                    match SavePathOptions::check(&self.save_path) {
                                        SavePathOptions::Ok => 
                                            Text::new("Valid path.")
                                                .color(Color::from([0.3, 0.7, 0.3])),
                                        SavePathOptions::InvalidExtension => 
                                            Text::new("Only \".jpg\" and \".png\" file types are allowed.")
                                                .color(Color::from([0.7, 0.3, 0.3])),
                                        SavePathOptions::NotFound => 
                                            Text::new("Parent directory does not exist.")
                                                .color(Color::from([0.7, 0.3, 0.3])),
                                        SavePathOptions::AlreadyExists => 
                                            Text::new("File already exists.")
                                                .color(Color::from([0.7, 0.3, 0.3]))
                                    }
                                )
                        )
                        .push(
                            Row::new()
                                .spacing(8)
                                .push(
                                    if LoadPathOptions::check(&self.load_path) == LoadPathOptions::Ok 
                                        && SavePathOptions::check(&self.save_path) == SavePathOptions::Ok
                                    {
                                        Button::new(&mut self.run_button, Text::new("Run"))
                                            .on_press(Message::Run)
                                            .padding(8)
                                            .border_radius(4)
                                            .background(Background::Color(Color::from([0.3, 0.7, 0.3])))
                                    } else {
                                        Button::new(&mut self.run_button, Text::new("Run"))
                                            .padding(8)
                                            .border_radius(4)
                                            .background(Background::Color(Color::from([0.7, 0.3, 0.3])))
                                    }
                                )
                                .push(
                                    Button::new(&mut self.exit_button, Text::new("Exit"))
                                        .on_press(Message::Exit)
                                        .padding(8)
                                        .border_radius(4)
                                        .background(Background::Color(Color::from([0.7, 0.7, 0.7])))
                                )
                        )
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
                },
                State::Finished(Ok(())) => {
                    Column::new()
                        .spacing(32)
                        .push(
                            Column::new()
                                .spacing(8)
                                .push(
                                    Text::new("Your Image is Ready")
                                        .size(32)
                                )
                                .push(
                                    Text::new(format!("The resulting image was saved under \"{}\".", self.save_path.display()))
                                )
                        )
                        .push(
                            Row::new()
                                .spacing(8)
                                .push(
                                    Button::new(&mut self.restart_button, Text::new("Restart"))
                                        .on_press(Message::Restart)
                                        .padding(8)
                                        .border_radius(4)
                                        .background(Background::Color(Color::from([0.7, 0.7, 0.7])))
                                )
                                .push(
                                    Button::new(&mut self.exit_button, Text::new("Exit"))
                                        .on_press(Message::Exit)
                                        .padding(8)
                                        .border_radius(4)
                                        .background(Background::Color(Color::from([0.7, 0.7, 0.7])))
                                )
                        )
                },
                State::Finished(Err(error)) => {
                    Column::new()
                        .spacing(32)
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
                        .push(
                            Row::new()
                                .spacing(8)
                                .push(
                                    Button::new(&mut self.restart_button, Text::new("Restart"))
                                        .on_press(Message::Restart)
                                        .padding(8)
                                        .border_radius(4)
                                        .background(Background::Color(Color::from([0.7, 0.7, 0.7])))
                                )
                                .push(
                                    Button::new(&mut self.exit_button, Text::new("Exit"))
                                        .on_press(Message::Exit)
                                        .padding(8)
                                        .border_radius(4)
                                        .background(Background::Color(Color::from([0.7, 0.7, 0.7])))
                                )
                        )
                }
            })
            .into()
    }
}