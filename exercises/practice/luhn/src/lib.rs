/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let without_spaces = code.replace(" ", "");
    let digits_only = without_spaces.chars().all(char::is_numeric);
    if !digits_only {
        return false;
    }

    const RADIX: u32 = 10;
    let digits: Vec<u32> = without_spaces.chars()
        .filter_map(|c| c.to_digit(RADIX))
        .rev()
        .collect();
    if digits.len() == 1 {
        return false;
    }
    let sum_of_doubled_digits: u32 = digits.iter().enumerate().map(|(i, &num)| {
        match i % 2 {
            0 => num,
            _ if num == 9 => num,
            _ => (num * 2) % 9, 
        }
    }).sum();

    sum_of_doubled_digits % 10 == 0
}
