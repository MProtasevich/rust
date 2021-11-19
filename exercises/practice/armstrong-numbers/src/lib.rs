pub fn is_armstrong_number(num: u32) -> bool {
    let num_of_digits = ((num as f64).log10().floor() + 1.0) as u32;
    num == (0..num_of_digits)
        .map(|i| ((num / 10u32.pow(i)) % 10).pow(num_of_digits))
        .sum::<u32>()
}
