extern crate clap;

mod file_system;
mod processors;

use clap::{App, Arg};
use std::path::PathBuf;

fn main() {
    let matches = App::new("Word Processor")
        .version("1.0")
        .author("Terrence Plunkett <eightys3v3n@gmail.com>")
        .about("Includes a number of utilities to process word/password lists")
        .arg(
            Arg::with_name("source_path")
                .short("s")
                .long("source-path")
                .takes_value(true)
                .help("Path to scan for new word/password list files"),
        )
        .arg(
            Arg::with_name("output_path")
                .short("o")
                .long("output-path")
                .takes_value(true)
                .help("Path to output the processed word/password list"),
        )
        .get_matches();

    let source_path = PathBuf::from(matches.value_of("source_path").unwrap_or("lists"));
    let output_path = PathBuf::from(matches.value_of("output_path").unwrap_or("output.lst"));

    if ! source_path.exists(source_path) {
        eprintln!("lists/ directory doesn't exist.");
        return 1
    }

    let words = file_system::read_files(&source_path);
    let words = processors::deduplicate(words);

    file_system::write_words(&output_path, &words);

    // Source path.
    // Read all files from source_path.
    // Output path; ask to overwrite.
}
