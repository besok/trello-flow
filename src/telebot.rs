use std::{collections::HashMap, fmt::format, io};

use crate::{
    err::FlowError,
    executor::{ConfigurationFiles, Executor},
    files::{read_file_into_string, yml_str_to},
};
use serde::{Deserialize, Serialize};
use teloxide::error_handlers::{IgnoringErrorHandler, OnError};

use teloxide::{
    dptree::HandlerResult, prelude::*, utils::command::BotCommands, ApiError, RequestError,
};
use tokio::task::JoinError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BotCred {
    token: String,
    task: String,
    board: String,
}

pub fn bot_from_file(bot_cred: &str) -> Result<Bot, FlowError> {
    let bot_cred: BotCred = yml_str_to(read_file_into_string(bot_cred)?.as_str())?;
    Ok(Bot::new(bot_cred.token))
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "run a task")]
    Task(String),
    #[command(description = "task list")]
    Tasks,
}

impl From<FlowError> for RequestError {
    fn from(value: FlowError) -> Self {
        RequestError::Api(ApiError::Unknown(format!("error: {:?}", value)))
    }
}

fn je_to_re(je: JoinError) -> RequestError {
    RequestError::Api(ApiError::Unknown(format!("error: {:?}", je)))
}

pub async fn processing(
    trello_info: ConfigurationFiles,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Tasks => {
            let mut tasks = tokio::spawn(async move {
                Ok::<_, FlowError>(Executor::from(trello_info, Default::default())?.tasks())
            })
            .await
            .map_err(je_to_re)??;

            tasks.sort();

            let tasks_str = tasks.join("\n");
            bot.send_message(msg.chat.id, tasks_str).await?;
        }
        Command::Task(command) => {
            let words: Vec<_> = command.split(" ").collect();
            let task = words.first().unwrap().to_string();
            let task_str = task.clone();
            let args: HashMap<String, String> = HashMap::from_iter(
                words[1..]
                    .into_iter()
                    .map(|s| s.split("=").map(|s| s.trim()).collect::<Vec<_>>())
                    .map(|v| (v[0].to_string(), v[1].to_string())),
            );

            let res = tokio::spawn(async move {
                let mut e = Executor::from(trello_info, args)?;
                e.start(task)
            })
            .await
            .map_err(je_to_re)??;

            bot.send_message(msg.chat.id, format!("the task {} is done.", task_str))
                .await?;
            bot.send_message(msg.chat.id, format!("{}", res.to_string()))
                .await?;
        }
    };

    Ok(())
}

pub async fn find_word(files: ConfigurationFiles, bot: Bot, msg: Message) -> ResponseResult<()> {
    let words: Vec<_> = msg
        .text()
        .expect("only text is allowed")
        .split(" ")
        .collect();
    let args: HashMap<String, String> = HashMap::from_iter(
        words
            .into_iter()
            .map(|s| s.split("=").map(|s| s.trim()).collect::<Vec<_>>())
            .map(|v| (v[0].to_string(), v[1].to_string())),
    );

    let bot_cred: BotCred = yml_str_to(read_file_into_string(files.bot.as_str())?.as_str())?;

    let res = tokio::spawn(async move {
        let mut e = Executor::from(files, args)?;
        e.start(bot_cred.task)
    })
    .await
    .map_err(je_to_re)??;

    bot.send_message(msg.chat.id, format!("{}", res.to_string()))
        .await?;
    Ok(())
}
