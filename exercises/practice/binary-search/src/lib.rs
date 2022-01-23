use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();
    while low < high {
        let idx = (high + low) / 2;
        match array[idx].cmp(&key) {
            Ordering::Less    => low = idx + 1,
            Ordering::Equal   => return Some(idx),
            Ordering::Greater => high = idx,
        };
    }

    None
}
