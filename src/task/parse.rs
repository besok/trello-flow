use std::{collections::HashMap, hash::Hash, vec};

use crate::err::{self, FlowError};

use yaml_rust::Yaml;

use super::tasks::{
    ActionTask, FilterTask, FlowTask, GroupTask, OrderTask, Place, Source, TakeTask, Target, Task,
    TaskBody,
};

#[derive(Clone)]
pub struct ParametrizedYaml<'a> {
    pub yaml: &'a Yaml,
    pub arguments: HashMap<String, String>,
}

impl<'a> From<&'a Yaml> for ParametrizedYaml<'a> {
    fn from(value: &'a Yaml) -> Self {
        ParametrizedYaml {
            yaml: value,
            arguments: HashMap::new(),
        }
    }
}

impl<'a> ParametrizedYaml<'a> {
    pub fn new(yaml: &'a Yaml, arguments: HashMap<String, String>) -> Self {
        Self { yaml, arguments }
    }
}

impl<'a> TryFrom<ParametrizedYaml<'a>> for Source {
    type Error = FlowError;

    fn try_from(value: ParametrizedYaml<'a>) -> Result<Self, Self::Error> {
        let src = field_by_name("source", value.clone()).and_then(as_string)?;
        match tpe(value)?.as_str() {
            "pipe" => Ok(Source::Pipe),
            "board" => Ok(Source::Board),
            "column" => Ok(Source::Column(src.to_string())),
            e => error(e),
        }
    }
}

impl<'a> TryFrom<ParametrizedYaml<'a>> for Place {
    type Error = FlowError;

    fn try_from(value: ParametrizedYaml<'a>) -> Result<Self, Self::Error> {
        field_by_name("place", value)
            .and_then(as_string)
            .and_then(|s| match s.as_str() {
                "top" => Ok(Place::Top),
                "bottom" => Ok(Place::Bottom),
                "random" => Ok(Place::Random),
                _ => error("place"),
            })
    }
}
impl<'a> TryFrom<ParametrizedYaml<'a>> for Target {
    type Error = FlowError;
    fn try_from(value: ParametrizedYaml<'a>) -> Result<Self, Self::Error> {
        let column = field_by_name("column", value.clone())
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

impl<'a> TryFrom<ParametrizedYaml<'a>> for TaskBody {
    type Error = FlowError;

    fn try_from(value: ParametrizedYaml<'a>) -> Result<Self, Self::Error> {
        let task_type = tpe(value.clone())?;
        let params = params(value)?;

        match task_type.as_str() {
            "group" => Ok(TaskBody::Group(GroupTask {
                steps: as_vec_of_str(params)?,
            })),
            "flow" => Ok(TaskBody::Flow(FlowTask {
                steps: as_vec_of_str(params)?,
            })),
            "action" => {
                let action_type = tpe(params.clone())?;
                match action_type.as_str() {
                    "print" => Ok(TaskBody::Action(ActionTask::PrintToConsole)),
                    "copy" => {
                        let to: Target = field_by_name("to", params).and_then(|y| y.try_into())?;
                        Ok(TaskBody::Action(ActionTask::CopyToColumn(to)))
                    }
                    "move" => {
                        let to: Target = field_by_name("to", params).and_then(|y| y.try_into())?;
                        Ok(TaskBody::Action(ActionTask::MoveToColumn(to)))
                    }
                    _ => error(action_type.as_str()),
                }
            }
            "filter" => {
                let by = or_default(
                    field_by_name("by", params.clone()).and_then(as_string),
                    "name".to_string(),
                )?;
                let rhs = field_by_name("rhs", params.clone()).and_then(as_string)?;
                let case = or_default(field_by_name("case", params).and_then(as_bool), true)?;
                match by.as_str() {
                    "name" => Ok(TaskBody::Filter(FilterTask::Name(rhs.to_string(), case))),
                    "label" => Ok(TaskBody::Filter(FilterTask::Label(rhs.to_string(), case))),
                    _ => error("the field is not either name or label "),
                }
            }
            "order" => {
                let from: Source = or_default(
                    field_by_name("from", params.clone()).and_then(|from| from.try_into()),
                    Source::Pipe,
                )?;
                match tpe(params)?.as_str() {
                    "shuffle" => Ok(TaskBody::Order(OrderTask::Shuffle(from))),
                    "sort" => Ok(TaskBody::Order(OrderTask::Sort(from))),
                    "reverse" => Ok(TaskBody::Order(OrderTask::Reverse(from))),
                    t => error(t),
                }
            }
            "take" => {
                let src: Source = or_default(
                    field_by_name("from", params.clone()).and_then(|from| from.try_into()),
                    Source::Pipe,
                )?;
                let place = or_default(params.clone().try_into(), Place::Top)?;
                let size = or_default(field_by_name("size", params).and_then(as_i64), 0)? as usize;
                Ok(TaskBody::Take(TakeTask { src, size, place }))
            }
            _ => error(task_type.as_str()),
        }
    }
}

fn tpe<'a>(yml: ParametrizedYaml<'a>) -> Result<String, FlowError> {
    field_by_name("type", yml).and_then(as_string)
}
fn params<'a>(yml: ParametrizedYaml<'a>) -> Result<ParametrizedYaml<'a>, FlowError> {
    field_by_name("params", yml)
}

pub fn field_by_name<'a>(
    name: &'a str,
    yml: ParametrizedYaml<'a>,
) -> Result<ParametrizedYaml<'a>, FlowError> {
    if let Yaml::Hash(h) = yml.yaml {
        h.get(&Yaml::String(name.to_string()))
            .map(|y| ParametrizedYaml::new(y, yml.arguments))
            .ok_or(FlowError::NoFieldError(format!(
                "{} is absent for {:?}",
                name, yml.yaml
            )))
    } else {
        Err(FlowError::SerdeError(format!(
            "error while deserializing for {:?}",
            yml.yaml
        )))
    }
}

fn par_yaml<'a, R, SR, YR>(
    v: ParametrizedYaml<'a>,
    fromStr: SR,
    fromYaml: YR,
) -> Result<R, FlowError>
where
    SR: Fn(&str) -> Result<R, FlowError>,
    YR: Fn(&Yaml) -> Result<R, FlowError>,
{
    if let Some(s) = v.yaml.as_str() {
        let mut unfolded_str = s.to_string();
        for (k, v) in v.arguments {
            let key = format!("~~{}~~", k);
            unfolded_str = unfolded_str.replace(&key, &v);
        }
        fromStr(&unfolded_str)
    } else {
        fromYaml(v.yaml)
    }
}

pub fn as_string<'a>(f: ParametrizedYaml<'a>) -> Result<String, FlowError> {
    par_yaml(
        f.clone(),
        |s| Ok(s.to_string()),
        |y| {
            y.as_str()
                .map(ToString::to_string)
                .ok_or(FlowError::SerdeError(format!(
                    "type should be string but got :{:?}",
                    f.yaml
                )))
        },
    )
}

pub fn as_bool<'a>(f: ParametrizedYaml<'a>) -> Result<bool, FlowError> {
    par_yaml(
        f.clone(),
        |s| {
            s.parse::<bool>()
                .map_err(|s| FlowError::SerdeError(s.to_string()))
        },
        |y| {
            y.as_bool()
                .ok_or(FlowError::SerdeError("should be bool".to_string()))
        },
    )
}
pub fn as_i64<'a>(f: ParametrizedYaml<'a>) -> Result<i64, FlowError> {
    par_yaml(
        f.clone(),
        |s| {
            s.parse::<i64>()
                .map_err(|s| FlowError::SerdeError(s.to_string()))
        },
        |y| {
            y.as_i64()
                .ok_or(FlowError::SerdeError("size should be a number".to_string()))
        },
    )
}

pub fn as_vec_of_str<'a>(f: ParametrizedYaml<'a>) -> Result<Vec<String>, FlowError> {
    if let Some(elems) = f.yaml.as_vec() {
        let mut res = vec![];
        for e in elems.into_iter() {
            res.push(
                as_string(ParametrizedYaml::new(e, f.arguments.clone())).map(|s| s.to_string())?,
            )
        }
        Ok(res)
    } else {
        Err(FlowError::SerdeError(format!(
            "type should be a vec but got :{:?}",
            f.yaml
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fmt::Debug};

    use yaml_rust::{Yaml, YamlLoader};

    use crate::{
        files::yml_str_to,
        task::{self, tasks::*},
    };

    use super::ParametrizedYaml;

    fn success<'a, T>(yaml: ParametrizedYaml<'a>, expected: T)
    where
        T: TryFrom<ParametrizedYaml<'a>> + Debug + PartialEq,
        <T as TryFrom<ParametrizedYaml<'a>>>::Error: Debug,
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
            (&yaml(
                r#"
        type: action
        params:
            type: move
            to:
                column: repeat
                place: top
        "#,
            ))
                .into(),
            TaskBody::Action(ActionTask::MoveToColumn(Target {
                column: "repeat".to_string(),
                place: Place::Top,
            })),
        );
        success(
            (&yaml(
                r#"
        type: action
        params:
            type: move
            to:
                column: repeat
        "#,
            ))
                .into(),
            TaskBody::Action(ActionTask::MoveToColumn(Target {
                column: "repeat".to_string(),
                place: Place::Top,
            })),
        );
    }

    #[test]
    fn flow() {
        success(
            (&yaml(
                r#"
        type: flow
        params:
            - repeat_others
            - repeat_mispronounced
            - repeat_demand
            - shuffle_idioms
        "#,
            ))
                .into(),
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
            (&yaml(
                r#"
        type: group
        params:
            - repeat_others
            - repeat_mispronounced
            - repeat_demand
            - shuffle_idioms
        "#,
            ))
                .into(),
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
    #[test]
    fn args() {
        let yaml = &yaml(
            r#"
        type: group
        params:
            - repeat_others
            - ~~arg1~~
            - repeat_~~arg2~~
            - ~~arg3~~_idioms
            - some_task~~arg4~~and_others
        "#,
        );
        let p_yaml = ParametrizedYaml::new(
            yaml,
            HashMap::from_iter(vec![
                ("arg1".to_string(), "repeat_mispronounced".to_string()),
                ("arg2".to_string(), "demand".to_string()),
                ("arg3".to_string(), "shuffle".to_string()),
                ("arg4".to_string(), "-complex_".to_string()),
            ]),
        );

        success(
            p_yaml,
            TaskBody::Group(GroupTask {
                steps: vec![
                    "repeat_others".to_string(),
                    "repeat_mispronounced".to_string(),
                    "repeat_demand".to_string(),
                    "shuffle_idioms".to_string(),
                    "some_task-complex_and_others".to_string(),
                ],
            }),
        );
    }
}
