use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
use std::collections::HashSet;
use log::{create_log, Log, UrlCount};
use itertools::sorted;
pub mod log;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
#[derive(Debug)]
struct UrlCount {
    url:String,
    count: i64
}
impl PartialEq for UrlCount{
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for UrlCount{}
impl Hash for UrlCount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}
impl Ord for UrlCount{
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}
impl PartialOrd for UrlCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args = Cli::from_args();
    let f = BufReader::new(File::open(&args.path).expect("could not read file"));
    let re = Regex::new(r#""([^"])*"|\s"#).unwrap();
    let mut logs: Vec<Log> = Vec::new();
    let mut log_set = HashSet::<UrlCount>::new();

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

    for log in logs{
        let url_info = UrlCount{url:log.request_string, count:1};
        if log_set.contains(&url_info){
            let actual_info = log_set.get(&url_info).expect("what");
            log_set.replace(UrlCount{url:url_info.url, count:actual_info.count + 1});
        }else{
            log_set.insert(url_info);
        }
    }
    
    for url_count in sorted(log_set){
        println!("Count: {} - URL:{} ", url_count.count, url_count.url);
    }
}
