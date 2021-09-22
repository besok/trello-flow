use std::fs::File;
use std::io::{Read, Error};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
#[derive(Debug)]
struct DictManager {
    cfg: Configuration,
    data:Vec<Record>
}

impl DictManager {
    fn new(cfg_path: &str, data_path:&str) -> DictManager {
        let json = read_file_into_string(cfg_path)
            .expect("should be presented");
        let cfg: Configuration = serde_json::from_str::<Configuration>(&json)
            .expect("the cfg should be correct");


        let mut reader =
            csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path(PathBuf::from(data_path)).expect("");

        let mut data:Vec<Record> = vec![];

        for r in reader.deserialize(){
            let record:Record = r.expect("the file with data should be correct");
            data.push(record)
        }

        DictManager {
            cfg, data
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Record {
    from: String,
    to: String,
    src: String,
    dst: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dictionary {
    name: String,
    board: String,
    list_create: String,
    list_move: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    key: String,
    token: String,
    dicts: Vec<Dictionary>,
}

pub fn read_file_into_string(path: &str) -> Result<String, Error> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

mod tests {
    use crate::file_manager::{DictManager, Configuration};

    #[test]
    fn cfg_test() {
        let cfg = r#"
            {
              "key":"6ee6c85be50cf98a9d06ff25fdaf6809",
              "token": "9d046d8b9565a78846e49c233b2cd14518a46b454995e915eb9c70f5c2d6c835",
              "dicts": [
                {"name": "German","board":"GER","list_create": "Later", "list_move": "Daily"},
                {"name": "English","board":"ENG","list_create": "Later", "list_move": "Daily"}
              ]
            }
        "#;
        let cfg = serde_json::from_str::<Configuration>(&cfg).unwrap();
        println!("{:?}", cfg)
    }

    #[test]
    fn file_test(){
        let dm = DictManager::new(
            "/Users/boriszhguchev/Documents/cfg.json",
                                  "/Users/boriszhguchev/Documents/1.csv");
        println!("{:?}",dm)
    }
}