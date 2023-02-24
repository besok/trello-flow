mod process;

use std::collections::HashMap;

use crate::{
    err::FlowError,
    files::read_file_into_string,
    task::{context::TaskContext, *},
    trello::{self, *},
};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Debug)]
pub struct ConfigurationFiles {
    pub trello: String,
    pub tasks: String,
    pub bot: String,
}
impl ConfigurationFiles {
    pub fn new(
        trello_cred: String,
        tasks: String,
        bot: String,
    ) -> Result<ConfigurationFiles, FlowError> {
        Ok(Self {
            trello: trello_cred,
            tasks,
            bot,
        })
    }
}

pub struct Executor {
    board_id: String,
    args: HashMap<String, String>,
    ctx: TaskContext,
    pub connector: TrelloConnector,
    rand: ThreadRng,
}

impl Executor {
    pub fn tasks(&self) -> Vec<String> {
        self.ctx.tasks.keys().map(Clone::clone).collect()
    }

    pub fn from(
        cfg: ConfigurationFiles,
        arguments: HashMap<String, String>,
    ) -> Result<Executor, FlowError> {
        Executor::from_files(cfg.trello.as_str(), cfg.tasks.as_str(), arguments)
    }
    pub fn from_files(
        cred_file: &str,
        yml_file: &str,
        arguments: HashMap<String, String>,
    ) -> Result<Executor, FlowError> {
        let ctx = context::from_str(read_file_into_string(yml_file)?.as_str(), arguments.clone())?;
        let connector = TrelloConnector::from_file(cred_file)?;
        Executor::new(ctx, connector, arguments)
    }

    fn new(
        ctx: TaskContext,
        connector: TrelloConnector,
        args: HashMap<String, String>,
    ) -> Result<Executor, FlowError> {
        let board_name = ctx.board.clone();
        let board = connector
            .boards()
            .into_iter()
            .find(|x| x.name == board_name)
            .ok_or(error("board is not found".to_string()));

        Ok(Self {
            ctx,
            board_id: board?.id,
            args,
            rand: rand::thread_rng(),
            connector,
        })
    }
    pub fn start(&mut self, task: String) -> Result<State, FlowError> {
        let task = self
            .ctx
            .tasks
            .get(&task)
            .map(Clone::clone)
            .ok_or(error(format!("a task {} is not found", task)))?;

        info!("the executor starts a task: {:?}", task);
        task.body.process(self, State::Init)
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Pipe(Vec<Card>),
    Init,
    End,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Pipe(e) => {
                let c_names: Vec<String> = e
                    .into_iter()
                    .map(|c| format!("{} {}", c.name, c.short_url))
                    .collect();
                let c_names = c_names.join("\n");
                if c_names.is_empty() {
                    "no cards found".to_string()
                } else {
                    c_names
                }
            }
            State::Init => "init".to_string(),
            State::End => "end".to_string(),
        }
    }
}

pub fn error(mes: String) -> FlowError {
    FlowError::ProcessingError(mes)
}

impl State {
    pub fn cards(&self) -> Result<Vec<Card>, FlowError> {
        match self {
            State::Pipe(elems) => Ok(elems.clone()),
            s => Err(FlowError::ProcessingError(format!(
                "no pipe results, state is {:?}",
                s
            ))),
        }
    }

    pub fn upd_cards(&self, cards: Vec<Card>) -> State {
        match self {
            State::Pipe(elems) => State::Pipe(cards),
            State::Init => State::Pipe(cards),
            State::End => State::Pipe(cards),
        }
    }
}

trait TaskProcessor {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError>;
}

#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn base_test() {
        let mut e = Executor::from_files(
            "/home/besok/projects/trello-flow/examples/trello_cred.yml",
            "/home/besok/projects/trello-flow/examples/task.yml",
            Default::default(),
        )
        .unwrap();

        let r = e.start("repeat".to_string()).unwrap();
    }
}
