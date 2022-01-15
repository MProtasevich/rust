use std::iter::{once, successors };

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut number = n;
    let factors = once(2).chain(successors(Some(3), |n: &u64| n.checked_add(2)))
        .take_while(|&factor| factor <= n);

    for factor in factors {
        while number % factor == 0 {
            prime_factors.push(factor);
            number /= factor;
        }
        if number == 1 {
            break;
        }
    }

    prime_factors
}
