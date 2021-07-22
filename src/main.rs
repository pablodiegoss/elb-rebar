use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
use std::collections::HashSet;
use log::{create_log, Log, UrlCount};
use itertools::sorted;
pub mod log;
use std::path::PathBuf;
use std::fs::metadata;
use walkdir::WalkDir;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn get_log_paths(mut file_paths: Vec<PathBuf>, path:&PathBuf) -> Vec<PathBuf>{
    let md = metadata(path).unwrap();
    println!("is dir: {}", md.is_dir());
    println!("is file: {}", md.is_file());
    if md.is_file() {
        file_paths.push(path.to_path_buf());
        file_paths
    }else{
        for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with(".log") {
                // println!("{}", f_name);
                // println!("{:?}", entry.path());
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }
}

fn main() {
    let total_results = 10;
    let args = Cli::from_args();
    let mut file_paths = Vec::<PathBuf>::new();
    let log_regex = Regex::new(r#""([^"])*"|\s"#).unwrap();
    file_paths = get_log_paths(file_paths, &args.path);
    let mut log_set = HashSet::<UrlCount>::new();
    for file in file_paths{
        let f = BufReader::new(File::open(file).expect("could not read file"));
        
        let mut logs: Vec<Log> = Vec::new();
        
        for line in f.lines() {
            let log_values = line
            .unwrap()
            .split_inclusive(&log_regex)
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
                let new_info = UrlCount{url:url_info.url, count:actual_info.count + 1};
                log_set.replace(new_info);
            }else{
                log_set.insert(url_info);
            }
        }        
    }
    let result:Vec<UrlCount> = sorted(log_set).collect();
    for url_count in &result[result.len()-total_results..]{
        println!("Count: {} - URL:{} ", url_count.count, url_count.url);
    }
    
}
