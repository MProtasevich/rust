use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate = candidate.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let set = candidate.iter().collect::<HashSet<_>>();
    set.len() == candidate.len()
}
