use std::str::FromStr;

pub fn encode(source: &str) -> String {
    let mut encoded_string: String = String::new();

    let mut occurences = 1;
    let mut peekable = source.chars().peekable();

    while let Some(chr) = peekable.next() {
        match peekable.peek() {
            Some(&next_char) if chr == next_char => occurences += 1,
            Some(_) | None => {
                encoded_string.push_str(format_substring(chr, occurences).as_str());
                occurences = 1;
            }
        }
    }

    encoded_string
}

fn format_substring(chr: char, occurrences: u32) -> String {
    if occurrences > 1 {
        format!("{}{}", occurrences, chr)
    } else {
        chr.to_string()
    }
}

pub fn decode(source: &str) -> String {
    source.split_inclusive(|ch: char| ch.is_alphabetic() || ch.is_ascii_whitespace())
        .map(|substring| (substring.strip_suffix(|ch: char| ch.is_alphabetic() || ch.is_ascii_whitespace()), substring.chars().last()))
        .map(|(count, ch)| (count.map(usize::from_str).unwrap().unwrap_or(1), ch.unwrap()))
        .map(|(count, ch)| std::iter::repeat(ch).take(count).collect::<String>())
        .collect()
}
