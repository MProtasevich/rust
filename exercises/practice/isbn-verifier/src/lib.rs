const RADIX: u32 = 10;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_digits = isbn.chars()
        .filter(|&chr| chr.is_digit(RADIX) || chr == 'X');

    let digits_count = isbn_digits.clone().count();
    let x_found = isbn.find('X');
    if digits_count != 10 || (x_found != None && x_found.unwrap() != isbn.len() - 1) {
        return false;
    }

    let hash_sum = isbn_digits
        .map(|chr| if chr == 'X' { 10 } else { chr.to_digit(RADIX).unwrap() as usize })
        .rev()
        .enumerate()
        .fold(0, |acc, (multiplier, x)| acc + x * (multiplier + 1));

    hash_sum % 11 == 0
}
