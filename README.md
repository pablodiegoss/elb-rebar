# Rust's AWS Elastic Load Balancer (ELB) - Log Analyzer

The Amazon ELB provides a huge amount of access logs, on which you can locate IPs of requests, processing times, URLs being requested, HTTP methods and statuses of each request. Rebar is a tool to help you analyze logs by providing quick insights into your ELB logs.
Rebar is parallel and focused on performance, this allows the user to analyze months worth of logs in a few minutes.

## Build

Cargo provides all the basic installation and usage for this repository.
If you haven't installed Rust and cargo yet, [click here](https://www.rust-lang.org/tools/install) and follow the instructions to set it up.

Building:

```bash
    cargo build
```

To execute Rebar from this repository you can pass arguments through `cargo`, we accept a direct log file or folder:

```bash
$ cargo run -- examples/example1.log
$ cargo run -- examples/
```

## Installation & Usage

You can install Rebar through `cargo`:

```bash
$ cargo install elb-rebar
```

We have `/examples/` for you to test during development or after installation:

```bash
$ elb-rebar examples/
$ elb-rebar examples/example1.log
```

Rebar will count and list all requests registered in your log file or folder (recursively).

#### Example output

```
$ elb-rebar examples/
-- Checking files to process --
-- Starting to process 4 files --


-- Results --
#  - Count - URL
1  - 14043 - "POST https://example.com.br:443/url2/
2  - 638   - "POST https://example.com.br:443/url5/
3  - 550   - "POST https://example.com.br:443/url1/
4  - 528   - "POST https://example.com.br:443/url3/
5  - 94    - "POST https://example.com.br:443/url4/

Finished in : 174ms
```
