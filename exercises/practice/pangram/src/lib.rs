use std::collections::HashSet;

const NUMBER_OF_LATIN_LETTERS: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<_>>().len() == NUMBER_OF_LATIN_LETTERS
}
