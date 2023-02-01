use crate::{
    err::FlowError,
    trello::{self, *},
};
use rand::{rngs::ThreadRng, Rng};

use super::{context::TaskContext, tasks::*};

struct Executor {
    board_id: String,
    ctx: TaskContext,
    state: State,
    connector: TrelloConnector,
    rand: ThreadRng,
}

impl Executor {
    // fn execute(&mut self, task: String) -> Result<State, FlowError> {}

    fn new(ctx: TaskContext, connector: TrelloConnector) -> Result<Executor, FlowError> {
        let board_name = ctx.board.clone();
        let board = connector
            .boards()
            .into_iter()
            .find(|x| x.name == board_name)
            .ok_or(error("board is not found".to_string()));

        Ok(Self {
            ctx,
            board_id: board?.id,
            state: State::Init,
            rand: rand::thread_rng(),
            connector,
        })
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Pipe(Vec<Card>, usize),
    Init,
    End,
}

fn error(mes: String) -> FlowError {
    FlowError::ProcessingError(mes)
}

impl State {
    pub fn cards(&self) -> Result<Vec<Card>, FlowError> {
        match self {
            State::Pipe(elems, _) => Ok(elems.clone()),
            _ => Err(FlowError::ProcessingError("no pipe results".to_string())),
        }
    }
    pub fn upd_cards(&self, cards: Vec<Card>) -> State {
        match self {
            State::Pipe(elems, idx) => State::Pipe(cards, idx + 1),
            State::Init => State::Pipe(cards, 0),
            State::End => State::Pipe(cards, 0),
        }
    }
}

trait TaskProcessor {
    fn process(&self, executor: &mut Executor) -> Result<State, FlowError>;
}

// impl TaskProcessor for TakeTask {
//     fn process(&self, executor: &mut Executor) -> Result<State, FlowError> {
//         let b_id = executor.board_id;
//         let entities = match self.src {
//             Source::Pipe => executor.state.cards()?,
//             Source::Board => executor.connector.cards(&executor.board_id),
//             Source::Column(name) => {
//                 let list = executor
//                     .connector
//                     .list_by_name(&b_id, &name)
//                     .ok_or(error(format!("the column is not found")))?;

//                 executor.connector.cards(&executor.board_id).into_iter()
//                 .filter(|c|c.),
//             }
//         };
//         match self.size {
//             i if i > 0 && i < entities.len() => ,
//             _ => Ok(executor.state.upd_cards(entities)),
//         }
//     }
// }
