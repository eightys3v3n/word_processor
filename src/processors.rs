use std::string::String;
use std::vec::Vec;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;

/// Remove duplicated words by sorting them.
pub fn deduplicate(mut words: Vec<String>) -> Vec<String> {
    let set: HashSet<_> = words.drain(..).collect(); // dedup
    words = set.into_iter().collect();
    words
}

pub fn remove_contains_symbols(words: Vec<String>) -> Vec<String> {
    fn is_letters(string: &String) -> bool {
        for char in string.chars() {
            if !char.is_alphabetic() {
                return false;
            }
        }
        return true;
    }

    words.into_iter().filter(is_letters).collect()
}

pub fn remove_lacking_symbols(words: Vec<String>) -> Vec<String> {
    fn is_letters(string: &String) -> bool {
        for char in string.chars() {
            if !char.is_alphabetic() {
                return true;
            }
        }
        return false;
    }

    words.into_iter().filter(is_letters).collect()
}

pub fn remove_outside_lengths(words: Vec<String>, min: usize, max: usize) -> Vec<String> {
    words
        .into_iter()
        .filter(|i| (i.len() >= min) & (i.len() <= max))
        .collect()
}

fn trim(word: String, c: char) -> String {
    let mut word = String::from(word);
    
    while word.starts_with(c) {
        word = match word.get(1..) {
            None => String::from(""),
            Some(v) => String::from(v),
        };
    }
    while word.ends_with(c) {
        word = match word.get(..word.len()-1) {
            None => String::from(""),
            Some(v) => String::from(v),
        };
    }
    return word;
}

pub fn trim_whitespaces(words: Vec<String>) -> Vec<String> {
    fn trim_whitespace(word: String) -> String {
        let word = trim(word, ' ');
        let word = trim(word, '\n');
        let word = trim(word, '\t');
        return word;
    }

    words
        .into_iter()
        .map(trim_whitespace)
        .collect()
}

pub fn remove_counts(words: Vec<String>) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+\s+(.*)").unwrap();
    }

    pub fn remove_count(mut word: String) -> String {
        if RE.is_match(&word) {
            let cap = RE.captures(&word).unwrap();
            
            word = String::from(cap.get(1).unwrap().as_str());
        }

        return word;
    }
    
    words.into_iter()
        .map(remove_count)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deduplicate() {
        let words: Vec<String> = vec!["Hello", "Goodbye", "Hello", "aa", "aa", "aa", "bb", "aa"]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["Goodbye", "Hello", "aa", "bb"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut result = deduplicate(words);
        result.sort_unstable();

        assert_eq!(result, correct);
    }

    #[test]
    fn test_remove_contains_symbols() {
        let words: Vec<String> = vec!["aaaa-", "bbbb", "cccc+", "dddd"]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["bbbb", "dddd"].into_iter().map(String::from).collect();

        let result = remove_contains_symbols(words);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_remove_lacking_symbols() {
        let words: Vec<String> = vec!["aaaa-", "bbbb", "cccc+", "dddd"]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["aaaa-", "cccc+"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = remove_lacking_symbols(words);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_remove_outside_length() {
        let words: Vec<String> = vec!["a", "ab", "abc", "abcd", "abcde", "abcdef"]
            .into_iter()
            .map(String::from)
            .collect();

        let correct: Vec<String> = vec!["abc", "abcd", "abcde"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = remove_outside_lengths(words, 3, 5);

        assert_eq!(result, correct);
    }

    #[test]
    fn test_trim_sepcific() {
        assert_eq!(trim(String::from(" Hello   "), ' '), String::from("Hello"));
        assert_eq!(trim(String::from("-Hello---"), '-'), String::from("Hello"));
    }

    #[test]
    fn test_trim_whitespaces() {
        let words: Vec<String> = vec![" Hello", "  Hello  ", "Hello", "\nHello\t", "\n\nHello  "]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["Hello", "Hello", "Hello", "Hello", "Hello"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = trim_whitespaces(words);
        assert_eq!(result, correct);
    }

    #[test]
    fn test_remove_counts() {
        let words: Vec<String> = vec!["4 Hello", "120321 Password", "23124     PASSWORD", "Password", "123Password", "123 123Password1"]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["Hello", "Password", "PASSWORD", "Password", "123Password", "123Password1"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = remove_counts(words);
        assert_eq!(result, correct);
    }
}
