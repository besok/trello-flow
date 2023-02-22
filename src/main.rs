mod err;
mod executor;
mod files;
mod task;
mod telebot;
mod trello;

use env_logger::Env;
use executor::ConfigurationFiles;
use telebot::{bot_from_file, find_word, processing, Command};

use teloxide::prelude::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let bot = bot_from_file("/home/besok/projects/trello-flow/examples/bot.yml")
        .expect("the files should exist");
    let cfg_files = ConfigurationFiles::new(
        "/home/besok/projects/trello-flow/examples/trello_cred.yml".to_string(),
        "/home/besok/projects/trello-flow/examples/task.yml".to_string(),
    )
    .expect("the files should exist");

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(processing),
        )
        .branch(dptree::endpoint(find_word));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![cfg_files])
        .build()
        .dispatch()
        .await;
}
