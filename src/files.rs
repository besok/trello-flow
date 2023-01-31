use serde::Deserialize;
use std::fs::File;
use std::io::{Error, Read};

pub fn read_file_into_string(path: &str) -> Result<String, Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn yml_str_to<'a, T: Deserialize<'a>>(from: &'a str, expect: &'static str) -> T {
    serde_yaml::from_str(&from).expect(expect)
}
