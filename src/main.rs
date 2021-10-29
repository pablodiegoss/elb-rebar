pub mod log;
pub mod utils;

use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};

use log::Log;
use utils::*;

use chrono::NaiveTime;
use structopt::StructOpt;
use threadpool::ThreadPool;

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
