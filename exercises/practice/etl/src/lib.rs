use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&k, v_list)| v_list.iter().map(move |&v| (v.to_ascii_lowercase(), k)))
        .collect()
}
