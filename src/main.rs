use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let file_result = File::open(args.path);

    let message = match file_result {
        Ok(file) => print_lines_containing(&args.pattern, file),
        Err(error) => format!("File error: {}", error)
    };

    println!("{}", message)
}

fn print_lines_containing(pattern: &String, file: File) -> String {
    let reader = BufReader::new(file);
    let matching_strings: Vec<String> = reader.lines()
        .map(Result::unwrap)
        .filter(|line| line.contains(pattern))
        .collect();

    matching_strings.join("\n")
}