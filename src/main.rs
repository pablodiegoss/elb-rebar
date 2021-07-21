use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

use log::{create_log, Log};
pub mod log;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let f = BufReader::new(File::open(&args.path).expect("could not read file"));
    let re = Regex::new(r#""([^"])*"|\s"#).unwrap();
    let mut logs: Vec<Log> = Vec::new();
    for line in f.lines() {
        let log_values = line
            .unwrap()
            .split_inclusive(&re)
            .map(|x| x.trim().replace('"', ""))
            .filter(|x| x != "")
            .collect::<Vec<String>>();
        let log = create_log(log_values);
        logs.push(log);
    }
}
