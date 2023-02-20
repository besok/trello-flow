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
