use std::io::{self, Error};

use yaml_rust::ScanError;

#[derive(Debug)]
pub enum FlowError {
    EnvError(Error),
    ParseError(ScanError),
    SerdeError(String),
    NoFieldError(String),
    UnexpectedValueError(String),
    ProcessingError(String),
}

impl From<Error> for FlowError {
    fn from(value: Error) -> Self {
        FlowError::EnvError(value)
    }
}

impl From<ScanError> for FlowError {
    fn from(value: ScanError) -> Self {
        FlowError::ParseError(value)
    }
}
impl From<serde_yaml::Error> for FlowError {
    fn from(value: serde_yaml::Error) -> Self {
        FlowError::SerdeError(
            value
                .location()
                .map(|l| format!("error in {}", l.index()))
                .unwrap_or("error in serde".to_string()),
        )
    }
}
