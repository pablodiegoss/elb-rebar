pub mod log;

use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    fs::metadata,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};

use log::Log;

use chrono::{format::ParseResult, NaiveTime, TimeZone, Utc};
use regex::Regex;
use structopt::StructOpt;
use threadpool::ThreadPool;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct ParseTimeError;

impl std::fmt::Display for ParseTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid time")
    }
}

fn from_time(src: &str) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(src, "%H%M")
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(default_value = "20", short = "l", long = "limit")]
    limit: i32,
    #[structopt(short = "it", long = "init_time", parse(try_from_str = from_time))]
    init_time: Option<NaiveTime>,
    #[structopt(short = "et", long = "end_time", parse(try_from_str = from_time))]
    end_time: Option<NaiveTime>,
}

fn validate_time(
    file_name: Cow<'_, str>,
    init_time: &Option<NaiveTime>,
    end_time: &Option<NaiveTime>,
) -> bool {
    let mut result = !(init_time.is_some() || end_time.is_some());
    let re = Regex::new(r"\d{8}T\d{4}Z").unwrap();
    let captures = re.captures_iter(&file_name);

    if let Some(c) = &captures.last() {
        let datetime = Utc.datetime_from_str(&c[0], "%Y%m%dT%H%MZ").unwrap();

        if init_time.is_some() {
            result = datetime.time() >= init_time.unwrap();
        }

        if end_time.is_some() {
            result = datetime.time() <= end_time.unwrap();
        }
    }

    result
}

fn get_log_paths(
    mut file_paths: Vec<PathBuf>,
    path: &Path,
    init_time: &Option<NaiveTime>,
    end_time: &Option<NaiveTime>,
) -> Vec<PathBuf> {
    let md = metadata(path).unwrap();
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
            if f_name.ends_with(".log") && validate_time(f_name, init_time, end_time) {
                // println!("{}", f_name);
                // println!("{:?}", entry.path());
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }
}

fn main() {
    let ncpus = num_cpus::get();
    let thread_pool = ThreadPool::new(ncpus * 2);
    let time = Instant::now();
    let args = Cli::from_args();
    let log_map = Arc::new(Mutex::new(HashMap::<String, u64>::new()));
    let mut file_paths = Vec::<PathBuf>::new();

    println!("-- Checking files to process --");
    file_paths = get_log_paths(file_paths, &args.path, &args.init_time, &args.end_time);

    println!("-- Starting to process {} files --", file_paths.len());
    for path in file_paths {
        let all_logs = Arc::clone(&log_map);
        thread_pool.execute(move || {
            let mut thread_log_map = HashMap::<String, u64>::new();

            let f = fs::read_to_string(path).expect("could not read file");
            f.lines().for_each(|line| {
                let log = line.split(' ').collect::<Log>();

                thread_log_map
                    .entry(format!("{} {}", log.request_method, log.request_url))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            });

            for (key, value) in thread_log_map.iter() {
                all_logs
                    .lock()
                    .unwrap()
                    .entry(key.to_owned())
                    .and_modify(|e| *e += value)
                    .or_insert(*value);
            }
        });
    }

    thread_pool.join();

    let all_logs = log_map.lock().unwrap();
    let mut count_vec: Vec<(&String, &u64)> = all_logs.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n\n-- Results --");
    println!("{:<2} - {:<5} - {:<3}", "#", "Count", "URL");
    for (i, (url, count)) in count_vec.iter().enumerate() {
        if i as i32 == args.limit && args.limit < count_vec.len() as i32 {
            println!(
                "\n... {} omitted.\nTo see more results use -l or --limit to a desired value.",
                count_vec.len() - i
            );
            break;
        }

        println!("{:<2} - {:<5} - {}", i + 1, count, url);
    }
    println!("\nFinished in : {}ms", time.elapsed().as_millis());
}
