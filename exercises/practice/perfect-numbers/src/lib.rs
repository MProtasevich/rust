use Classification::{ Deficient, Perfect, Abundant };
use std::cmp::Ordering::{ Less, Equal, Greater };

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum_of_divisors: u64 = (1..=(num / 2))
        .filter(|&divisor| num % divisor == 0)
        .sum();

    let classification = match sum_of_divisors.cmp(&num) {
        Less => Deficient,
        Equal => Perfect,
        Greater => Abundant,
    };

    Some(classification)
}
