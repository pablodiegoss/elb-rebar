use structopt::StructOpt;

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

    for line in f.lines() {
        for value in line.unwrap().split(' ') {
            println!("{}", value);
        }
    }
    println!("Hello, world!");
}
