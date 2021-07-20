use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Log {
    request_type: String, // http, https, h2
    close_time: String,   // Timestamp in ISO 8601 format
    elb_identifier: String,
    client: String,                // IP:port
    target: String,                // IP:port
    request_processing_time: f32, // in seconds, with millisecond precision for load balancer -> target. Can be -1 if the connection closed
    target_processing_time: f32, // in seconds, with millisecond precision for target -> load balancer. Can be -1 if the connection closed
    response_processing_time: f32, // in seconds, with millisecond precision for load balancer -> client. Can be -1 if the connection closed
    elb_status_code: i32,
    target_status_code: i32,
    received_bytes: i64,    // The size of the request, in bytes.
    sent_bytes: i64,        // The size of the response, in bytes.
    request_string: String, // enclosed in double quotes. formatted as HTTP method + protocol://host:port/uri + HTTP version eg. "POST https://example.com.br:443/url1/ HTTP/2.0"
    user_agent: String,     // enclosed in double quotes.
    ssl_cipher: String,
    ssl_protocol: String,
    target_group_arn: String,
    x_amazn_trace_id: String,      // enclosed in double quotes.
    sni_domain_name: String,       // enclosed in double quotes.
    cert_arn: String,              // enclosed in double quotes.
    matched_rule_priority: i32,    // a value from 1 to 50,000, default rule is 0.
    request_creation_time: String, // ISO 8601 format.
    actions_executed: String,      // enclosed in double quotes
    redirect_url: String,          // enclosed in double quotes
    error_reason: String,          //enclosed in double quotes
    target_list: String,           //enclosed in double quotes
    classification: String,        //enclosed in double quotes
    classification_reason: String, //enclosed in double quotes
}

fn main() {
    let args = Cli::from_args();
    let f = BufReader::new(File::open(&args.path).expect("could not read file"));
    let re = Regex::new(r#""([^"])*"|\s"#).unwrap();

    for line in f.lines() {
        for value in line
            .unwrap()
            .split_inclusive(&re)
            .map(|x| x.trim().replace('"', ""))
            .filter(|x| x != "")
            .collect::<Vec<String>>()
        {
            println!("{}", value);
        }
    }
}
