mod app;
mod operator;

use std::{
    path::{
        PathBuf
    },
};
use iced::{
    Settings,
    settings::Window,
    Application
};
use app::App;
use operator::{
    Operator,
    Load,
    Position,
    Join,
    Save
};

fn main() {
    let mut operator = Operator::default();
    operator.add(Load::new(PathBuf::from("images")));
    operator.add(Position::new());
    operator.add(Join::new());
    operator.add(Save::new(PathBuf::from("result.jpg")));
    operator.run();

    App::run(Settings {
        window: Window {
            size: (640, 480),
            resizable: false
        }
    });
}