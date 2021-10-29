use chrono::{format::ParseResult, NaiveTime, TimeZone, Utc};
use regex::Regex;
use std::{
    borrow::Cow,
    fs::metadata,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn from_time(src: &str) -> ParseResult<NaiveTime> {
    NaiveTime::parse_from_str(src, "%H%M")
}

pub fn validate_time(
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

pub fn get_log_paths(
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
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }
}
