# Word Processor.
This is a small collection of word/password list processing utilities I have used in my escapades. The word list and word transformation functions are in processors.rs. The app logic and the order of processor use is in main.rs. All file system functions are in file_system.rs.
This also happens to be my first Rust project! Commit ef2ad6f50474f91a9ca9804051fc21829d8cb800 took me 5h20m to complete (including all the learning).

## Getting started
Open up main.rs and decide what you want to do. Then have a look in processors.rs to see if there are functions to do everything you want to do. If not, write some to do it and make a pull request. Then change the order of processing under `let words = file_system::read_files(files);` but before `file_system::write_words(&output_path, &words);`.
Then run the program using -s to specify what folder to load files from and -o for the generated output file.

## Projects I've used this for
- Processing 1.4G of password lists into a single list containing over 30,000,000 unique passwords.

## Things to consider
- Profile the program as it takes way longer than a similar Python program.
- Time different deduplication methods (sorting then dedup vs iterator deduplication).
- Add some better doc-strings to everything.
- Would be nice if it had a text language to specify the transformations instead of having to recompile.
- A help page for command line options (not sure if clap includes that).

## Password Utilities
This is the Python project I translated into Rust. It was written in a hurry during classes and while it worked, it had no documentation or testing. It still has one function more than this program.

## English Word Frequencies
This counts the frequencies of words in the source texts. It was used to find the most used words, then find the most likely password for a word combination lock. At some point I would like to translate this program into Rust as well.
