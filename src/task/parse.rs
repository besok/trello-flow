use std::vec;

use crate::err::{self, FlowError};
use crate::task::TaskBody;
use yaml_rust::Yaml;

use super::{
    ActionTask, FilterTask, FlowTask, GroupTask, OrderTask, Place, Source, TakeTask, Target, Task,
};

impl TryFrom<&Yaml> for Source {
    type Error = FlowError;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        let src = field_by_name("source", value).and_then(as_string);
        match tpe(value)? {
            "pipe" => Ok(Source::Pipe),
            "board" => Ok(Source::Board),
            "column" => Ok(Source::Column(src?.to_string())),
            e => error(e),
        }
    }
}

impl TryFrom<&Yaml> for Place {
    type Error = FlowError;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        field_by_name("place", value)
            .and_then(as_string)
            .and_then(|s| match s {
                "top" => Ok(Place::Top),
                "bottom" => Ok(Place::Bottom),
                "random" => Ok(Place::Random),
                _ => error("place"),
            })
    }
}
impl TryFrom<&Yaml> for Target {
    type Error = FlowError;
    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        let column = field_by_name("column", value)
            .and_then(as_string)
            .map(|s| s.to_string())?;

        let place = or_default(value.try_into(), Place::Top)?;

        Ok(Target { column, place })
    }
}
fn error<T>(t: &str) -> Result<T, FlowError> {
    Err(FlowError::UnexpectedValueError(format!(
        "the type '{}' is not recognized",
        t
    )))
}

fn or_default<T>(e: Result<T, FlowError>, default: T) -> Result<T, FlowError> {
    match e {
        Err(FlowError::NoFieldError(_)) => Ok(default),
        err => err,
    }
}

impl TryFrom<&Yaml> for TaskBody {
    type Error = FlowError;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        let task_type = tpe(value)?;
        let params = params(value)?;

        match task_type {
            "group" => Ok(TaskBody::Group(GroupTask {
                steps: as_vec_of_str(params)?,
            })),
            "flow" => Ok(TaskBody::Flow(FlowTask {
                steps: as_vec_of_str(params)?,
            })),
            "action" => {
                let action_type = tpe(params)?;
                match action_type {
                    "print" => Ok(TaskBody::Action(ActionTask::PrintToConsole)),
                    "copy" => {
                        let to: Target = field_by_name("to", params).and_then(|y| y.try_into())?;
                        Ok(TaskBody::Action(ActionTask::CopyToColumn(to)))
                    }
                    "move" => {
                        let to: Target = field_by_name("to", params).and_then(|y| y.try_into())?;
                        Ok(TaskBody::Action(ActionTask::MoveToColumn(to)))
                    }
                    _ => error(action_type),
                }
            }
            "filter" => {
                let by = or_default(field_by_name("by", params).and_then(as_string), "name")?;
                let rhs = field_by_name("by", params).and_then(as_string)?;
                let case = or_default(
                    field_by_name("case", params).and_then(|cs| match cs.as_bool() {
                        Some(v) => Ok(v),
                        None => Err(FlowError::SerdeError("should be bool".to_string())),
                    }),
                    false,
                )?;
                match by {
                    "name" => Ok(TaskBody::Filter(FilterTask::Name(rhs.to_string(), case))),
                    "label" => Ok(TaskBody::Filter(FilterTask::Label(rhs.to_string(), case))),
                    _ => error("the field is not either name or label "),
                }
            }
            "order" => {
                let from: Source = or_default(
                    field_by_name("from", params).and_then(|from| from.try_into()),
                    Source::Pipe,
                )?;
                match tpe(params)? {
                    "shuffle" => Ok(TaskBody::Order(OrderTask::Shuffle(from))),
                    "sort" => Ok(TaskBody::Order(OrderTask::Sort(from))),
                    "reverse" => Ok(TaskBody::Order(OrderTask::Reverse(from))),
                    t => error(t),
                }
            }
            "take" => {
                let src: Source = or_default(
                    field_by_name("from", params).and_then(|from| from.try_into()),
                    Source::Pipe,
                )?;
                let place = or_default(params.try_into(), Place::Top)?;
                let size = or_default(
                    field_by_name("size", params).and_then(|e| match e.as_i64() {
                        Some(v) => Ok(v),
                        None => error("size should be a number"),
                    }),
                    0,
                )?;
                Ok(TaskBody::Take(TakeTask { src, size, place }))
            }
            _ => error(task_type),
        }
    }
}

fn tpe(yml: &Yaml) -> Result<&str, FlowError> {
    field_by_name("type", yml).and_then(as_string)
}
fn params(yml: &Yaml) -> Result<&Yaml, FlowError> {
    field_by_name("params", yml)
}

pub fn field_by_name<'a>(name: &'a str, yml: &'a Yaml) -> Result<&'a Yaml, FlowError> {
    if let Yaml::Hash(h) = yml {
        h.get(&Yaml::String(name.to_string()))
            .ok_or(FlowError::NoFieldError(format!(
                "{} is absent for {:?}",
                name, yml
            )))
    } else {
        Err(FlowError::SerdeError(format!(
            "error while deserializing for {:?}",
            yml
        )))
    }
}

pub fn as_string(f: &Yaml) -> Result<&str, FlowError> {
    f.as_str().ok_or(FlowError::SerdeError(format!(
        "type should be string but got :{:?}",
        f
    )))
}
pub fn as_vec_of_str(f: &Yaml) -> Result<Vec<String>, FlowError> {
    if let Some(elems) = f.as_vec() {
        let mut res = vec![];
        for e in elems.into_iter() {
            res.push(as_string(e).map(|s| s.to_string())?)
        }
        Ok(res)
    } else {
        Err(FlowError::SerdeError(format!(
            "type should be a vec but got :{:?}",
            f
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use yaml_rust::{Yaml, YamlLoader};

    use crate::{
        files::yml_str_to,
        task::{self, ActionTask, FlowTask, GroupTask, Target, TaskBody},
    };

    fn success<'a, T>(yaml: &'a Yaml, expected: T)
    where
        T: TryFrom<&'a Yaml> + Debug + PartialEq,
        <T as TryFrom<&'a Yaml>>::Error: Debug,
    {
        let actual: T = yaml.try_into().unwrap();
        assert_eq!(actual, expected);
    }

    fn yaml(task: &str) -> Yaml {
        YamlLoader::load_from_str(task)
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }

    #[test]
    fn action() {
        success(
            &yaml(
                r#"
        type: action
        params:
            type: move
            to:
                column: repeat
                place: top
        "#,
            ),
            TaskBody::Action(ActionTask::MoveToColumn(Target {
                column: "repeat".to_string(),
                place: task::Place::Top,
            })),
        );
        success(
            &yaml(
                r#"
        type: action
        params:
            type: move
            to:
                column: repeat
        "#,
            ),
            TaskBody::Action(ActionTask::MoveToColumn(Target {
                column: "repeat".to_string(),
                place: task::Place::Top,
            })),
        );
    }

    #[test]
    fn flow() {
        success(
            &yaml(
                r#"
        type: flow
        params:
            - repeat_others
            - repeat_mispronounced
            - repeat_demand
            - shuffle_idioms
        "#,
            ),
            TaskBody::Flow(FlowTask {
                steps: vec![
                    "repeat_others".to_string(),
                    "repeat_mispronounced".to_string(),
                    "repeat_demand".to_string(),
                    "shuffle_idioms".to_string(),
                ],
            }),
        );
    }

    #[test]
    fn group() {
        success(
            &yaml(
                r#"
        type: group
        params:
            - repeat_others
            - repeat_mispronounced
            - repeat_demand
            - shuffle_idioms
        "#,
            ),
            TaskBody::Group(GroupTask {
                steps: vec![
                    "repeat_others".to_string(),
                    "repeat_mispronounced".to_string(),
                    "repeat_demand".to_string(),
                    "shuffle_idioms".to_string(),
                ],
            }),
        );
    }
}
