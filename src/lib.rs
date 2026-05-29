use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<String>,
    entry_type: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("findr")
	.version("0.1.0")
	.about("Rust find")
	.arg(
	    Arg::with_name("paths")
		.value_name("PATH")
		.help("Search paths")
		.default_value(".")
		.multiple(true),
	)
	.arg(
	    Arg::with_name("names")
		.value_name("NAME")
		.short("n")
		.long("name")
		.help("Name")
		.takes_value(true)
		.multiple(true),
	)
	.arg(
	    Arg::with_name("types")
		.value_name("TYPE")
		.short("t")
		.long("type")
		.help("Entry type")
		.possible_values(&["f", "d", "l"])
		.takes_value(true)
		.multiple(true),
	)
	.get_matches();

    Ok(Config {
	paths: Vec::new(),
	names: Vec::new(),
	entry_type: Vec::new(),
    })
}

pub fn run(config: &Config) -> MyResult<()> {
    Err("hogehoge".into())
}
