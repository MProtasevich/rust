#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: vec![(std::cmp::min(a, b), std::cmp::max(a, b))],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        if a * b != self.value {
            panic!("Product of provided factors doesn't equal to the Palindrome value")
        }
        self.factors.push((a, b))
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome = Palindrome::new(min, max);
    let mut max_palindrome = Palindrome::new(0, 0);

    for i in min..=max {
        for j in i..=max {
            let v = i * j;
            if min_palindrome.value == v {
                min_palindrome.insert(i, j);
            } else if min_palindrome.value > v && is_palindrome(v) {
                min_palindrome = Palindrome::new(i, j);
            }
            if max_palindrome.value == v {
                max_palindrome.insert(i, j);
            } else if max_palindrome.value < v && is_palindrome(v) {
                max_palindrome = Palindrome::new(i, j);
            }
        }
    }
    if max_palindrome.value > 0 {
        Some((min_palindrome, max_palindrome))
    } else {
        None
    }
}

fn is_palindrome(value: u64) -> bool {
    let mut reversed = 0;
    let mut copied = value;
    while copied > 0 {
        reversed *= 10;
        reversed += copied % 10;
        copied /= 10;
    }
    reversed == value
}
