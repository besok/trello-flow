use serde::Deserialize;
use std::fs::File;
use std::io::{Error, Read};

use crate::err::FlowError;

pub fn read_file_into_string(path: &str) -> Result<String, Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn yml_str_to<'a, T: Deserialize<'a>>(from: &'a str) -> Result<T, FlowError> {
    serde_yaml::from_str(&from).map_err(|e| e.into())
}
