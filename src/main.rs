mod gui;
mod cli;
mod operator;
mod error;

use iced::{
    Settings,
    settings::Window,
    Application
};

fn main() {
    if cfg!(feature = "gui") {
        println!("Running as GUI application.");
        gui::App::run(Settings {
            window: Window {
                size: (640, 480),
                resizable: false
            }
        });
    } else {
        println!("Running as CLI application.");
        cli::run();
    }
}