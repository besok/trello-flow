use std::fs::File;
use std::io::{Read, Error};
use serde::Deserialize;
pub fn read_file_into_string(path: &str) -> Result<String, Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn cfg_json_into<'a, T: Deserialize<'a>>(from: &'a str, expect: &'static str) ->  T {
    serde_json::from_str(&from).expect(expect)
}