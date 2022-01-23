#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0, _) | (1, _) => return Err(Error::InvalidInputBase),
        (_, 0) | (_, 1) => return Err(Error::InvalidOutputBase),
        (_, _) => (),
    };
    if let Some(&digit) = number.iter().find(|&&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(digit));
    }

    let mut num = number.iter().rev().enumerate()
        .fold(0, |acc, (exp, digit)| acc + from_base.pow(exp as u32) * digit);

    if num == 0 {
        return Ok(vec![0]);
    }

    let mut result_vec = vec![];
    while num != 0 {
        let x = num % to_base;
        result_vec.push(x);
        num /= to_base;
    }

    result_vec.reverse();
    Ok(result_vec)
}
