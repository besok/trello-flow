use clap::{App, Arg};
use crate::extractor::Extractor;
use crate::executor::Executor;

mod trello;
mod dict;
mod extractor;
mod files;
mod matcher;
mod executor;

fn main() {
    let args = create_args().get_matches();
    let cred = args.value_of("cred").expect("credentials should exist");
    let data = args.value_of("data").expect("data should exist");
    let cfg = args.value_of("cfg").expect("cfg should be");
    Executor::execute(Extractor::new_from(cfg, cred, data))
}

fn create_args<'a,'b>() -> App<'a,'b> {
    App::new("trello-dict-updater")
        .version("0.1")
        .author("Boris Zhguchev <zhguchev@gmail.com>")
        .help("Update trello cards from the csv file.")
        .arg(
            Arg::with_name("cred")
                .long("cred")
                .takes_value(true)
                .allow_hyphen_values(true)
                .help("the file credentials for trello account"))
        .arg(
            Arg::with_name("cfg")
                .long("cfg")
                .takes_value(true)
                .allow_hyphen_values(true)
                .help("cfg for dictionaries"))
        .arg(
            Arg::with_name("data")
                .long("data")
                .takes_value(true)
                .help("data to upload"))
}