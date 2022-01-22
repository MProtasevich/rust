pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|factor| (1u32..).map(move |i| i * factor).take_while(|n| n < &limit))
        .collect::<std::collections::HashSet<_>>()
        .iter().sum()
}
