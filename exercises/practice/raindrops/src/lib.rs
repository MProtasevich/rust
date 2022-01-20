const DROPS: [(u32, &'static str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong"),
];

pub fn raindrops(n: u32) -> String {
    let res: String = DROPS.iter()
        .filter(|&&(num, _)| n % num == 0)
        .map(|&(_, drop)| drop)
        .collect();

    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
