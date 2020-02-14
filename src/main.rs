extern crate clap;

mod file_system;
mod processors;

use std::io;
use std::io::Write;
use clap::{App, Arg};
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

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

    println!("Reading lines from files...");
    let words = file_system::read_files(files);
    println!("Found {} words.", words.len());

    println!("Trimming leading and tailing whitespace...");
    let words = processors::trim_whitespaces(words);
    
    print!("Removing counts...");
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let words = processors::remove_counts(words);
    println!(" {}ms", now.elapsed().as_millis());
    
    print!("Deduplicating...");
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let words = processors::deduplicate(words);
    println!(" {}ms", now.elapsed().as_millis());
    println!("Found {} unique words.", words.len());

    println!("Removing any words longer than 50 characters... (some of the lists I sourced off Github had HTML pointing to their site)");
    let words = processors::remove_outside_lengths(words, 0, 50);
    println!("{} words left.", words.len());
    
    println!("Saving words...");
    file_system::write_words(&output_path, &words);
}

// TODO: Profile this beast.
