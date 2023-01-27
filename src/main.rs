use gui::Todos;
use iced::{window, Application, Settings};

mod dict;
mod executor;
mod extractor;
mod files;
mod gui;
mod matcher;
mod trello;

fn main() {
    Todos::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    });
}
