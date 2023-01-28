use executor::Executor;
use gui::Todos;
use iced::{window, Application, Settings};

mod executor;
mod files;
mod gui;
mod trello;

fn main() {
    Executor::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    });
}
