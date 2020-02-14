extern crate clap;

mod file_system;
mod processors;

use clap::{App, Arg};
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let extensions: Vec<&str> = vec!["txt", "lst"];

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

    if !source_path.exists() {
        eprintln!("lists/ directory doesn't exist.");
        exit(1);
    }

    println!("Getting file list...");
    let files = file_system::list_files(&source_path, true);

    println!("Getting valid files...");
    let files = file_system::filter_extensions(files, extensions);

    println!("Reading files...");
    let words = file_system::read_files(files);

    println!("Deduplicating...");
    let words = processors::deduplicate(words);

    println!("Saving words...");
    file_system::write_words(&output_path, &words);

    // Source path.
    // Read all files from source_path.
    // Output path; ask to overwrite.
}
