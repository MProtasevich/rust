use std::collections::{ HashSet, HashMap };

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn hash_word(word: &str) -> HashMap<char, u16> {
        word.to_lowercase().chars().fold(HashMap::new(), |mut dict, chr| {
            dict.entry(chr).and_modify(|e| { *e += 1 }).or_insert(1);
            dict
        })
    }
    let hashed_word = hash_word(word);
    possible_anagrams.iter()
        .filter(|anagram| anagram.to_lowercase() != word.to_lowercase())
        .filter(|anagram| hash_word(anagram) == hashed_word)
        .map(|&string| string)
        .collect()
}
