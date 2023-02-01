use std::collections::HashMap;
use std::io::{Error, Read};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use yaml_rust::YamlLoader;

use super::parse::as_string;
use super::tasks::{
    ActionTask, FilterTask, FlowTask, GroupTask, OrderTask, Place, Source, TakeTask, Target, Task,
    TaskBody,
};
use crate::err::FlowError;
use crate::files::read_file_into_string;
use crate::trello::Card;

#[derive(Debug, Clone)]
pub struct TaskContext {
    pub board: String,
    pub tasks: HashMap<String, Task>,
}

impl Default for TaskContext {
    fn default() -> Self {
        Self {
            board: Default::default(),
            tasks: Default::default(),
        }
    }
}

pub fn from_str(yml: &str) -> Result<TaskContext, FlowError> {
    let yml_content = read_file_into_string(yml)?;
    let yamls = YamlLoader::load_from_str(&yml_content)?;

    let yaml = yamls
        .first()
        .and_then(|s| s.as_hash())
        .ok_or(FlowError::SerdeError("the doc is empty".to_string()))?;

    let mut tasks: HashMap<String, Task> = HashMap::new();
    let mut board = String::new();

    for (k, v) in yaml.into_iter() {
        match as_string(k)? {
            "board" => board = as_string(v)?.to_string(),
            e => {
                let body: TaskBody = v.try_into()?;
                let task = Task {
                    name: e.to_string(),
                    body,
                };
                tasks.insert(e.to_string(), task);
            }
        }
    }

    Ok(TaskContext { board, tasks })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::task::{tasks::*, *};

    use super::from_str;

    #[test]
    fn test() {
        let ctx = from_str("/home/bzhg/projects/trello-vocab-loader/examples/task.yml").unwrap();
        assert_eq!(ctx.board, "ENG".to_string());

        assert_eq!(
            ctx.tasks["shuffle_idioms"],
            Task {
                name: "shuffle_idioms".to_string(),
                body: TaskBody::Order(OrderTask::Shuffle(Source::Column("Idioms".to_string())))
            }
        );
        assert_eq!(
            ctx.tasks["take_from_archive"],
            Task {
                name: "take_from_archive".to_string(),
                body: TaskBody::Take(TakeTask {
                    src: Source::Column("archive".to_string()),
                    size: 0,
                    place: Place::Top
                })
            }
        );
        assert_eq!(
            ctx.tasks["filter_demand"],
            Task {
                name: "filter_demand".to_string(),
                body: TaskBody::Filter(FilterTask::Label("demand".to_string(), true))
            }
        );
        assert_eq!(
            ctx.tasks["filter_mispronounced"],
            Task {
                name: "filter_mispronounced".to_string(),
                body: TaskBody::Filter(FilterTask::Label("mispronounced".to_string(), true))
            }
        );
        assert_eq!(
            ctx.tasks["take_5"],
            Task {
                name: "take_5".to_string(),
                body: TaskBody::Take(TakeTask {
                    src: Source::Pipe,
                    size: 5,
                    place: Place::Random
                })
            }
        );
        assert_eq!(
            ctx.tasks["take_10"],
            Task {
                name: "take_10".to_string(),
                body: TaskBody::Take(TakeTask {
                    src: Source::Pipe,
                    size: 10,
                    place: Place::Random
                })
            }
        );
        assert_eq!(
            ctx.tasks["move_to_repeat"],
            Task {
                name: "move_to_repeat".to_string(),
                body: TaskBody::Action(ActionTask::MoveToColumn(Target {
                    column: "repeat".to_string(),
                    place: Place::Top
                }))
            }
        );
        assert_eq!(
            ctx.tasks["repeat_demand"],
            Task {
                name: "repeat_demand".to_string(),
                body: TaskBody::Flow(FlowTask {
                    steps: vec![
                        "take_from_archive".to_string(),
                        "filter_demand".to_string(),
                        "take_5".to_string(),
                        "move_to_repeat".to_string(),
                    ]
                })
            }
        );
        assert_eq!(
            ctx.tasks["repeat_mispronounced"],
            Task {
                name: "repeat_mispronounced".to_string(),
                body: TaskBody::Flow(FlowTask {
                    steps: vec![
                        "take_from_archive".to_string(),
                        "filter_mispronounced".to_string(),
                        "take_5".to_string(),
                        "move_to_repeat".to_string(),
                    ]
                })
            }
        );
        assert_eq!(
            ctx.tasks["repeat_others"],
            Task {
                name: "repeat_others".to_string(),
                body: TaskBody::Flow(FlowTask {
                    steps: vec![
                        "take_from_archive".to_string(),
                        "take_10".to_string(),
                        "move_to_repeat".to_string(),
                    ]
                })
            }
        );
        assert_eq!(
            ctx.tasks["repeat"],
            Task {
                name: "repeat".to_string(),
                body: TaskBody::Group(GroupTask {
                    steps: vec![
                        "repeat_others".to_string(),
                        "repeat_mispronounced".to_string(),
                        "repeat_demand".to_string(),
                        "shuffle_idioms".to_string(),
                    ]
                })
            }
        );
    }
}
