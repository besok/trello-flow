use super::{error, Executor, State, TaskProcessor};
use crate::{
    err::FlowError,
    task::tasks::{
        ActionTask, FilterTask, FlowTask, GroupTask, OrderTask, Place, Source, TakeTask, Target,
        Task, TaskBody,
    },
    trello::List,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use std::{collections::HashSet, vec};

fn find_list(executor: &mut Executor, name: &str) -> Result<List, FlowError> {
    executor
        .connector
        .list_by_name(&executor.board_id, &name)
        .ok_or(error(format!("the column {} is not found", name)))
}

impl TaskProcessor for TaskBody {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        match self {
            TaskBody::Take(t) => t.process(executor, state),
            TaskBody::Order(t) => t.process(executor, state),
            TaskBody::Filter(t) => t.process(executor, state),
            TaskBody::Action(t) => t.process(executor, state),
            TaskBody::Group(t) => t.process(executor, state),
            TaskBody::Flow(t) => t.process(executor, state),
        }
    }
}
impl TaskProcessor for Source {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        let items = match &self {
            Source::Pipe => state.cards()?,
            Source::Board => executor.connector.cards(&executor.board_id),
            Source::Column(name) => {
                let list = find_list(executor, name)?;
                executor.connector.cards_in_list(&list.id)
            }
        };
        Ok(State::Pipe(items))
    }
}

impl TaskProcessor for ActionTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        match self {
            ActionTask::PrintToConsole => {
                let cards = state.cards()?;
                for c in cards.iter() {
                    println!("card:{:?}", c);
                }
                Ok(state)
            }
            ActionTask::CopyToColumn(trg) => todo!(),
            ActionTask::MoveToColumn(Target { column, place }) => {
                let lid = find_list(executor, &column)?;
                let cards = state.cards()?;
                match place {
                    Place::Top => cards.into_iter().for_each(|c| {
                        executor.connector.mov_card(&c.id, &lid.id, "top");
                    }),

                    Place::Bottom => cards.into_iter().for_each(|c| {
                        executor.connector.mov_card(&c.id, &lid.id, "bottom");
                    }),
                    Place::Random => todo!(),
                }
                Ok(State::End)
            }
        }
    }
}

impl TaskProcessor for GroupTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        for step in &self.steps {
            let _ = executor.start(step.clone())?;
        }
        Ok(State::End)
    }
}

impl TaskProcessor for TakeTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        let entities = self.src.process(executor, state)?.cards()?;
        let max = entities.len();
        match self.size {
            i if i > 0 && i < max => match self.place {
                Place::Top => Ok(State::Pipe(entities[..i].to_vec())),
                Place::Bottom => Ok(State::Pipe(entities[i..].to_vec())),
                Place::Random => {
                    let mut ids = HashSet::new();

                    while ids.len() != i {
                        let rand_idx = executor.rand.gen_range(0..max);
                        ids.insert(rand_idx);
                    }
                    let mut res = vec![];
                    for i in ids {
                        res.push(entities[i].clone())
                    }
                    Ok(State::Pipe(res))
                }
            },
            _ => Ok(State::Pipe(entities)),
        }
    }
}

impl TaskProcessor for FilterTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        let cards = state.cards()?;
        match self {
            FilterTask::Name(name, case) => Ok(State::Pipe(
                cards
                    .into_iter()
                    .filter(|c| {
                        if *case {
                            c.name == *name
                        } else {
                            c.name.to_lowercase() == name.to_lowercase()
                        }
                    })
                    .collect(),
            )),
            FilterTask::Label(label, case) => {
                let label = executor
                    .connector
                    .label_by_name(&executor.board_id, label, *case)
                    .map(|l| l.id)
                    .ok_or(error(format!("the label is not found")))?;
                Ok(State::Pipe(
                    cards
                        .into_iter()
                        .filter(|c| c.id_labels.contains(&label))
                        .collect(),
                ))
            }
        }
    }
}

impl TaskProcessor for OrderTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        let mut items = self.source()?.process(executor, state)?.cards()?;
        let mut items = match self {
            OrderTask::Shuffle(_s) => {
                items.shuffle(&mut executor.rand);
                items
            }
            OrderTask::Sort(_s) => {
                items.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                items
            }
            OrderTask::Reverse(_s) => {
                items.reverse();
                items
            }
        };
        Ok(State::Pipe(items))
    }
}

impl TaskProcessor for FlowTask {
    fn process(&self, executor: &mut Executor, state: State) -> Result<State, FlowError> {
        self.steps.iter().fold(Ok(state), |st, step| {
            let state = st?;
            let task = executor.ctx.task(step)?;
            task.body.process(executor, state)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        executor::{Executor, State, TaskProcessor},
        task::{
            context::from_str,
            tasks::{TakeTask, TaskBody},
        },
        trello::TrelloConnector,
    };

    fn trello() -> TrelloConnector {
        TrelloConnector::from_file(
            "/home/bzhg/projects/trello-vocab-loader/examples/trello_cred.yml",
        )
    }
    // #[test]
    // fn filter() {
    //     let ctx = from_str(
    //         r#"
    //         board: ENG
    //         filter_demand:
    //             type: filter
    //             params:
    //                 by: label
    //                 rhs: demand
    //     "#,
    //     )
    //     .unwrap();

    //     let mut e = Executor::new(ctx.clone(), trello()).unwrap();
    //     let task = ctx.tasks.get("filter_demand").unwrap().clone();

    //     let res = task.body.process(&mut e).unwrap();
    //     println!("{:?}", res.cards().unwrap());
    // }
    #[test]
    fn take() {
        let ctx = from_str(
            r#"
            board: ENG
            take_from_archive:
                type: take
                params:
                    from:
                        type: column
                        source: Archive
                    size: 10
                    place: random    
        "#,
            HashMap::new(),
        )
        .unwrap();

        let mut e = Executor::new(ctx.clone(), trello(), HashMap::new()).unwrap();
        let task = ctx.tasks.get("take_from_archive").unwrap().clone();

        let res = task.body.process(&mut e, State::Init).unwrap();
        println!("{:?}", res.cards().unwrap());
    }
    #[test]
    fn sort() {
        let ctx = from_str(
            r#"
            board: ENG
            shuffle_idioms:
                type: order
                params:
                    type: sort
                    from:
                        type: column # pipe by default and all from can be omitted
                        source: Idioms  
        "#,
            HashMap::new(),
        )
        .unwrap();

        let mut e = Executor::new(ctx.clone(), trello(), HashMap::new()).unwrap();
        let task = ctx.tasks.get("shuffle_idioms").unwrap().clone();

        let res = task.body.process(&mut e, State::Init).unwrap();
        println!("{:?}", res.cards().unwrap());
    }

    #[test]
    fn flow() {
        let ctx = from_str(
            r#"
            board: ENG
            take_from_archive:
                type: take
                params:
                    from:
                        type: column
                        source: Archive
                    size: 10
                    place: random 
            shuffle:
                type: order
                params:
                    type: sort
            
            flow_task:
                type: flow
                params:
                    - take_from_archive
                    - shuffle        
        "#,
            HashMap::new(),
        )
        .unwrap();

        let mut e = Executor::new(ctx.clone(), trello(), HashMap::new()).unwrap();
        let task = ctx.tasks.get("flow_task").unwrap().clone();

        let res = task.body.process(&mut e, State::Init).unwrap();
        let card_names: Vec<String> = res
            .cards()
            .unwrap()
            .iter()
            .map(|c| c.name.clone())
            .collect();

        println!("{:?}", card_names);
    }

    #[test]
    fn action_move() {
        let ctx = from_str(
            r#"
            board: ENG
            take_from_archive:
                type: take
                params:
                    from:
                        type: column
                        source: Archive
                    size: 10
                    place: random 
            shuffle:
                type: order
                params:
                    type: sort
            
            move:
                type: action
                params:
                    type: move
                    to:
                        column: Repeating
                        place: top
            flow:
                type: flow
                params:
                    -  take_from_archive
                    -  shuffle
                    -  move              
        "#,
            HashMap::new(),
        )
        .unwrap();

        let mut e = Executor::new(ctx.clone(), trello(), HashMap::new()).unwrap();
        let task = ctx.tasks.get("flow").unwrap().clone();

        let res = task.body.process(&mut e, State::Init).unwrap();
    }
}
