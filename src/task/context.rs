use std::collections::HashMap;
use std::io::{Error, Read};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use yaml_rust::YamlLoader;

use super::parse::as_string;
use super::{Task, TaskBody};
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

#[derive(Debug, Clone)]
pub enum Step<T> {
    Pipe(Vec<T>, usize),
    Init(String),
    End,
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
    use super::from_str;

    #[test]
    fn test() {
        let r = from_str("/home/bzhg/projects/trello-vocab-loader/examples/task.yml").unwrap();
        println!("{:?}", r);
    }
}
