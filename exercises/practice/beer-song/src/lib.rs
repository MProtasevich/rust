fn count_bottles(n: u32) -> String {
    match n {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{n} bottles", n = n),
    }
}

pub fn verse(n: u32) -> String {
    let we_had_bottles = count_bottles(n);
    let we_have_bottles = count_bottles(n.checked_sub(1).unwrap_or(99)).to_lowercase();
    let need_to_buy_more = if n == 0 {
        String::from("Go to the store and buy some more")
    } else {
        let take_one = if n == 1 { "it" } else { "one" };
        format!("Take {} down and pass it around", take_one)
    };
    let fst_sentence = format!("{} of beer on the wall, {} of beer.\n", we_had_bottles, we_had_bottles.to_lowercase());
    let scnd_sentence = format!("{}, {} of beer on the wall.\n", need_to_buy_more, we_have_bottles);
    fst_sentence + &scnd_sentence
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev()
        .map(|n| verse(n))
        .reduce(|acc, sentence| acc + "\n" + &sentence).unwrap()
}
