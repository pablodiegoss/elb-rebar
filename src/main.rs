use itertools::sorted;
use log::{create_log, Log, UrlCount};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use structopt::StructOpt;
pub mod log;
use std::fs::metadata;
use std::path::PathBuf;
use walkdir::WalkDir;
extern crate num_cpus;
extern crate threadpool;
use std::fs;
use std::thread;
use threadpool::ThreadPool;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn get_log_paths(mut file_paths: Vec<PathBuf>, path: &PathBuf) -> Vec<PathBuf> {
    let md = metadata(path).unwrap();
    println!("is dir: {}", md.is_dir());
    println!("is file: {}", md.is_file());
    if md.is_file() {
        file_paths.push(path.to_path_buf());
        file_paths
    } else {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
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

fn convert_lines_to_logs(file_path: &PathBuf) -> Vec<Log>{
    let mut logs: Vec<Log> = Vec::new();
    let log_regex = Regex::new(r#""([^"])*"|\s"#).unwrap();    

    let f = BufReader::new(File::open(file_path).expect("could not read file"));
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
    logs
}
fn main() {
    let ncpus = num_cpus::get();
    let thread_pool = ThreadPool::new(ncpus * 2);
    let time = Instant::now();
    println!("Starting - {}", time.elapsed().as_millis());
    // thread_pool.
    let total_results = 10;
    let args = Cli::from_args();
    let mut file_paths = Vec::<PathBuf>::new();
    file_paths = get_log_paths(file_paths, &args.path);
    let mut log_set = HashSet::<UrlCount>::new();
    let mut all_logs = Arc::new(Mutex::new(Vec::<Vec<Log>>::new()));
    for file in file_paths {
        let all_logs = Arc::clone(&all_logs);
        thread_pool.execute(move || {
            println!("Initializing work on {:?}", file);
            let thread_logs = convert_lines_to_logs(&file);
            all_logs.lock().unwrap().push(thread_logs);
        });
        // let logs = convert_lines_to_logs(&file);
    }
    thread_pool.join();
    println!(
        "Threads joining finished in : {}ms",
        time.elapsed().as_millis()
    );
    let mut logs = Vec::<Log>::new();
    for mut log_vec in all_logs.lock().unwrap().drain(..) {
        logs.append(&mut log_vec);
    }
    println!(
        "Joining vecs finished in : {}ms",
        time.elapsed().as_millis()
    );
    for log in logs {
        let url_info = UrlCount {
            url: log.request_string,
            count: 1,
        };
        if log_set.contains(&url_info) {
            let actual_info = log_set.get(&url_info).expect("what");
            let new_info = UrlCount {
                url: url_info.url,
                count: actual_info.count + 1,
            };
            log_set.replace(new_info);
        } else {
            log_set.insert(url_info);
        }
    }
    println!("URL counts finished in : {}ms", time.elapsed().as_millis());
    let result: Vec<UrlCount> = sorted(log_set).collect();
    let print_size = if result.len() > total_results {
        result.len() - total_results
    } else {
        0
    };

    for url_count in &result[print_size..] {
        println!("Count: {} - URL:{} ", url_count.count, url_count.url);
    }
    println!("Finished in : {}ms", time.elapsed().as_millis());
}
