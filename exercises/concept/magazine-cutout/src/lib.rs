// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

use std::collections::{
    HashMap,
    hash_map::Entry::{ Occupied, Vacant }
};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    fn fold_function<'a>(mut acc: HashMap<&'a str, i32>, elem: &&'a str) -> HashMap<&'a str, i32> {
        acc.entry(elem)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
        acc
    }

    let mut dictionary = magazine.iter()
        .fold(HashMap::new(), fold_function);

    for word in note {
        let entry = dictionary.entry(word).and_modify(|e| { *e -= 1 });
        match entry {
            Occupied(value) if value.get() < &0 => return false,
            Vacant(_) => return false,
            _ => continue
        }
    }

    true
}
