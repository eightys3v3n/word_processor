use std::string::String;
use std::vec::Vec;

/// Remove duplicated words by sorting them.
pub fn deduplicate(mut words: Vec<String>) -> Vec<String> {
    words.sort_unstable();
    words.dedup();
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

        let result = deduplicate(words);

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
}
