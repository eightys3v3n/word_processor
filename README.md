# Word Processor.
This is a small collection of word/password list processing utilities I have used in my escapades. The word list and word transformation functions are in processors.rs. The app logic and the order of processor use is in main.rs. All file system functions are in file_system.rs.
This also happens to be my first Rust project! Commit ef2ad6f50474f91a9ca9804051fc21829d8cb800 took me 5h20m to complete (including all the learning).

## Getting started
Open up main.rs and decide what you want to do. Then have a look in processors.rs to see if there are functions to do everything you want to do. If not, write some to do it and make a pull request. Then change the order of processing under `let words = file_system::read_files(files);` but before `file_system::write_words(&output_path, &words);`.
Then run the program using -s to specify what folder to load files from and -o for the generated output file.

### Example
At the time of writing, the main.rs contains this between read_files and write_words (excluding println!).
Words are a password, each stored on a new line.
```
    let words = processors::trim_whitespaces(words);
	// This will remove leading and trailing whitespace from all the words.

    let words = processors::remove_counts(words);
	// This will change password lists that have counts like '24 password' to 'password'.
	
    let words = processors::deduplicate(words);
	// This will remove all the duplicate words.

    let words = processors::remove_outside_lengths(words, 0, 50);
	// This removes any words that are shorter than 0 or longer than 50 in length.
```
Functions that take the `words: Vec<String>` argument are indented to be put where the example functions are.

Use `cargo build` to compile, `cargo test` to run the tests, and `cargo run -- -s <source_folder> -o <output_file>` to run the program.

## Projects I've used this for
- Processing 1.4G of password lists into a single list containing around 30,000,000 unique passwords (as well as emails and some random junk mixed in).

## Things to consider
- Profile the program as it takes way longer than a similar Python program.
- Time different deduplication methods (sorting then dedup vs iterator deduplication).
- Add some better doc-strings to everything.
- Would be nice if it had a text language to specify the transformations instead of having to recompile.
- A help page for command line options (not sure if clap includes that).

## SecLists
This is the source of the passwords that were used to create list.lst.gz. Everything inside SecLists/Passwords with some things removed (such as HTML tags to the rockyou website >:( )

## Password Utilities
This is the Python project I translated into Rust. It was written in a hurry during classes and while it worked, it had no documentation or testing. It still has one function more than this program.

## English Word Frequencies
This counts the frequencies of words in the source texts. It was used to find the most used words, then find the most likely password for a word combination lock. At some point I would like to translate this program into Rust as well.
