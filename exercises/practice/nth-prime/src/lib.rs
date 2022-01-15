fn is_prime(n: u32) -> bool {
    let prime_edge = (n as f64).sqrt().floor() as u32;
    n % 2 != 0 && (3..=prime_edge).step_by(2).all(|divider| n % divider != 0)
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        2
    } else {
        (3..)
            .step_by(2)
            .filter(|&n| is_prime(n))
            .take(n as usize)
            .last().unwrap()
    }
}
