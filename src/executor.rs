mod process;

use std::collections::HashMap;

use crate::{
    err::FlowError,
    task::{context::TaskContext, *},
    trello::{self, *},
};
use rand::{rngs::ThreadRng, Rng};

struct Executor {
    board_id: String,
    args: HashMap<String, String>,
    ctx: TaskContext,
    connector: TrelloConnector,
    rand: ThreadRng,
}

impl Executor {
    // fn execute(&mut self, task: String) -> Result<State, FlowError> {}

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
    fn start(&mut self, task: String) -> Result<State, FlowError> {
        let task = self
            .ctx
            .tasks
            .get(&task)
            .map(Clone::clone)
            .ok_or(error(format!("a task {} is not found", task)))?;
        task.body.process(self, State::Init)
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Pipe(Vec<Card>),
    Init,
    End,
}

pub fn error(mes: String) -> FlowError {
    FlowError::ProcessingError(mes)
}

impl State {
    pub fn cards(&self) -> Result<Vec<Card>, FlowError> {
        match self {
            State::Pipe(elems) => Ok(elems.clone()),
            _ => Err(FlowError::ProcessingError("no pipe results".to_string())),
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
