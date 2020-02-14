use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

pub fn list_files(root: &PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::<PathBuf>::new();

    for res in fs::read_dir(root).unwrap() {
        let path = match res {
            Err(why) => panic!("failed to read directory {}", why.description()),
            Ok(data) => data.path(),
        };

        if path.is_file() {
            files.push(path);
        } else if recursive & path.is_dir() {
            files.extend(list_files(&path, true));
        }
    }

    files.sort();

    files
}

/// Read a file, seperating words by newline characters.
fn read_lines(path: &PathBuf) -> Vec<String> {
    let display = path.display();

    let file = match fs::File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let mut contents: Vec<String> = vec![];

    for line in lines {
        let l: String = match line {
            Err(why) => {
                eprintln!("error in file {}: {}", display, why.description());
                String::from("")
            }
            Ok(data) => data,
        };

        if !l.is_empty() {
            contents.push(l);
        }
    }

    contents
}

/// Removes all files that don't have one of the given extensions.
pub fn filter_extensions(files: Vec<PathBuf>, exts: Vec<&str>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|f| match f.extension() {
            None => false,
            Some(ext) => match ext.to_str() {
                None => false,
                Some(ext) => exts.contains(&ext),
            },
        })
        .collect()
}

/// Reads all files found in the given path and returns all words.
pub fn read_files(files: Vec<PathBuf>) -> Vec<String> {
    let mut words: Vec<String> = Vec::<String>::new();

    for file in files {
        words.extend(read_lines(&file));
    }

    words
}

/// Write words, seperating by a newline character.
pub fn write_words(path: &PathBuf, words: &Vec<String>) {
    let sep = "\n";
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    for word in words {
        match file.write(format!("{}{}", word, sep).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(root: &str) {
        fn create_dir(path: &PathBuf) {
            let display = path.display();

            match fs::create_dir(path) {
                Err(why) => eprintln!(
                    "couldn't create directory {}: {}",
                    display,
                    why.description()
                ),
                Ok(_) => (),
            }
        }

        fn create_file(path: &PathBuf) {
            let display = path.display();

            match fs::File::create(path) {
                Err(why) => eprintln!("couldn't create file {}: {}", display, why.description()),
                Ok(_) => (),
            }
        }
        let paths = vec![
            ('d', "sub_dir"),
            ('f', "file_one"),
            ('f', "file_two"),
            ('f', "sub_dir/file_three"),
        ];
        let paths = paths
            .into_iter()
            .map(|p| (p.0, [root, p.1].iter().collect::<PathBuf>()));

        create_dir(&PathBuf::from(root));
        for p in paths {
            if p.0 == 'd' {
                create_dir(&p.1);
            } else if p.0 == 'f' {
                create_file(&p.1);
            }
        }
    }

    fn delete_tree(root: &str) {
        let path = PathBuf::from(root);
        let display = path.display();

        match fs::remove_dir_all(&path) {
            Err(why) => panic!(
                "failed to remove directory {}: {}",
                display,
                why.description()
            ),
            Ok(_) => (),
        }
    }

    #[test]
    fn test_list_files() {
        let root = "test_list_files";
        create_tree(root);

        let result = list_files(&PathBuf::from(root), false);
        let correct: Vec<PathBuf> = vec!["file_one", "file_two"]
            .into_iter()
            .map(|p| [root, p].iter().collect())
            .collect();

        delete_tree(root);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_filter_extensions() {
        let files: Vec<PathBuf> = vec!["file.txt", "file.jpeg", "file.lst"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let exts = vec!["txt", "lst"];
        let correct: Vec<PathBuf> = vec!["file.txt", "file.lst"]
            .into_iter()
            .map(PathBuf::from)
            .collect();

        let result = filter_extensions(files, exts);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_list_files_recursive() {
        let root = "test_list_files_recursive";
        create_tree(root);

        let result = list_files(&PathBuf::from(root), true);
        let correct: Vec<PathBuf> = vec!["file_one", "file_two", "sub_dir/file_three"]
            .into_iter()
            .map(|p| [root, p].iter().collect())
            .collect();

        delete_tree(root);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_read_lines() {
        let correct = vec!["Hello", "World", "How", "Are", "You?"];
        let content = "Hello\nWorld\nHow\nAre\nYou?\n";
        let path = PathBuf::from("test_read_lines.txt");
        let display = path.display();

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => (),
        }

        let results = read_lines(&path);

        match fs::remove_file(&path) {
            Err(why) => eprintln!(
                "couldn't remove testing file {}: {}",
                display,
                why.description()
            ),
            Ok(_) => (),
        };

        assert_eq!(results, correct);
    }

    #[test]
    fn test_read_empty_lines() {
        let correct = vec!["Hello", "World", "How", "Are", "You?"];
        let content = "Hello\nWorld\nHow\n\n\nAre\nYou?\n";
        let path = PathBuf::from("test_read_empty_lines.txt");
        let display = path.display();

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => (),
        }

        let results = read_lines(&path);

        match fs::remove_file(&path) {
            Err(why) => eprintln!(
                "couldn't remove testing file {}: {}",
                display,
                why.description()
            ),
            Ok(_) => (),
        };

        assert_eq!(results, correct);
    }

    #[test]
    fn test_write_words() {
        let correct = "Hello\nThere\nJorge\n";
        let words: Vec<String> = vec!["Hello", "There", "Jorge"]
            .into_iter()
            .map(String::from)
            .collect();
        let path = PathBuf::from("test_write_words.txt");
        let display = path.display();

        write_words(&path, &words);

        let result =
            fs::read_to_string(&path).expect(&format!("couldn't read output file {}", display));

        match fs::remove_file(&path) {
            Err(why) => eprintln!(
                "couldn't remove testing file {}: {}",
                display,
                why.description()
            ),
            Ok(_) => (),
        };

        assert_eq!(result, correct);
    }
}
