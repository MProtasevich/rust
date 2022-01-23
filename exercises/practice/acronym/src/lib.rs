pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '-', '_'][..])
        .flat_map(|word| vec![
            word.chars().nth(0),
            word.chars().skip(1).skip_while(|chr| chr.is_uppercase()).find(|chr| chr.is_uppercase())
        ].into_iter())
        .filter_map(std::convert::identity)
        .collect::<String>()
        .to_uppercase()
}
