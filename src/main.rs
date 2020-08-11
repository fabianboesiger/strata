#![windows_subsystem = "windows"]

mod app;
mod operator;

use iced::{
    Settings,
    window,
    Application,
};

fn main() {
    println!("Starting application.");
    app::App::run(Settings {
        window: window::Settings {
            size: (640, 480),
            resizable: false,
            ..
            Default::default()
        },
        ..
        Default::default()
    });
}