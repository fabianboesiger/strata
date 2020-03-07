mod app;
mod operator;
mod error;

use iced::{
    Settings,
    settings::Window,
    Application
};
use app::App;

fn main() {
    App::run(Settings {
        window: Window {
            size: (640, 480),
            resizable: false
        }
    });
}