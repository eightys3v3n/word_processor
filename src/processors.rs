use std::string::String;
use std::vec::Vec;
use std::collections::HashSet;

///  Returns a list of words, in random order, with all duplicates removed.
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["Hello", "World", "World"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = asd(input_words);
/// output_words.sort();
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
pub fn deduplicate(mut words: Vec<String>) -> Vec<String> {
    let set: HashSet<_> = words.drain(..).collect(); // dedup
    words = set.into_iter().collect();
    words
}

///  Returns a list of words, in an order reliant on .filter, with all words where char.is_alphabetic() is true for all characters.
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["Hello", "World", "Worl!d"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = asd(input_words);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
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

/// Returns a list of words, in an order reliant on .filter, with all words where char.is_alphabetic() is false for all characters..
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["Hell-o", "Wo@rld", "World"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = remove_lacking_symbols(input_words);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
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

/// Returns a list of words, in an order reliant on .map, where all words outside the given lengths are removed.
/// Both ends of the range are inclusive.
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["Hello", "World", "Password"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = remove_outside_lengths(input_words, 5, 5);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
pub fn remove_outside_lengths(words: Vec<String>, min: usize, max: usize) -> Vec<String> {
    words
        .into_iter()
        .filter(|i| (i.len() >= min) & (i.len() <= max))
        .collect()
}

/// Returns a list of words, in an order reliant on .map, with leading and trailing whitespace removed from all words.
/// Whitespace is determined by String.trim().
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec![" Hello ", "\nWorld  "]
///     .into_iter().map(String::from).collect();
///
/// let output_words = trim_whitespaces(input_words);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
pub fn trim_whitespaces(words: Vec<String>) -> Vec<String> {
    fn trim_whitespace(word: String) -> String {
        String::from(word.trim())
    }

    words
        .into_iter()
        .map(trim_whitespace)
        .collect()
}

/// Returns true if a String is all digits, false otherwise.
///
/// # Arguments
///
/// * `word` - The word to check.
///
/// # Example
///
/// ```
/// assert!( ! is_number(String::from("Hello")));
/// assert!(is_number(String::from("2134")));
/// ```
pub fn is_number(word: &String) -> bool {
    for c in word.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    return true;
}

/// Returns a list of words, in an order reliant on .map, with a leading number followed by a password; the two seperated by a any number of spaces.
/// Words that are processed are trimmed after the count is removed. However, only words where the first split grouping is a number are processed.
///
/// # Arguments
///
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["4 Hello ", "1230      World"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = remove_counts(input_words);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Hello");
/// assert!(output_words[1].as_str() == "World");
/// ```
pub fn remove_counts(words: Vec<String>) -> Vec<String> {
    let remove_count = |mut word: String| -> String {
        if word.len() < 3 { return word; } // it can't have a count and a password if it's too short
        if !word.contains(' ') {return word; }

        let splits: Vec<String> = word.split(' ')
            .into_iter()
            .map(String::from)
            .collect();
        if is_number(&splits[0]) && splits.len() > 1 {
            word = splits[1..].join(" ");
        }

        word = String::from(word.trim());
        return word;
    };
   
    words.into_iter()
        .map(remove_count)
        .collect()
}

/// Returns a list of words, in an order reliant on .map, where the prefix is inserted to every word.
///
/// # Arguments
///
/// * `prefix` - A string to insert before the word.
/// * `words` - A vector of all the words to process, one word per string.
///
/// # Example
///
/// ```
/// // Create an input list of words.
/// let input_words: Vec<String> = vec!["Hello ", "World"]
///     .into_iter().map(String::from).collect();
///
/// let output_words = prefix("Jorge ", input_words);
///
/// assert!(output_words.len() == 2);
/// assert!(output_words[0].as_str() == "Jorge Hello");
/// assert!(output_words[1].as_str() == "Jorge World");
/// ```
pub fn prefix(prefix: &str, words: Vec<String>) -> Vec<String> {
    let prefixed = |word: String| -> String {
        let prefixed_word: String = format!("{}{}", prefix, word);
        return prefixed_word;
    };
   
    words.into_iter()
        .map(prefixed)
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

    #[test]
    fn test_prefix() {
        let words: Vec<String> = vec!["Hello", "Password", "PASSWORD"]
            .into_iter()
            .map(String::from)
            .collect();
        let correct: Vec<String> = vec!["John Hello", "John Password", "John PASSWORD"]
            .into_iter()
            .map(String::from)
            .collect();
        let prefix: str = "John ";
        let result = prefix(prefix, words);
        assert_eq!(result, correct);
    }
    
    #[test]
    fn test_is_number() {
        assert_eq!(is_number(&String::from("hello")), false);
        assert_eq!(is_number(&String::from("1Helo")), false);
        assert_eq!(is_number(&String::from("123")), true);
        assert_eq!(is_number(&String::from("1 2")), false);
        assert_eq!(is_number(&String::from("1helo2")), false);
        assert_eq!(is_number(&String::from("039")), true);
    }
}
