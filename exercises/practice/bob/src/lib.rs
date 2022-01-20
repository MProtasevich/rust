pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let is_question = message.ends_with('?');
    let yelling = !message.is_empty() &&
                    message.chars().any(|chr| chr.is_ascii_alphabetic()) &&
                    message == message.to_uppercase();

    match (is_question, yelling, message.is_empty()) {
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (true, true, _)  => "Calm down, I know what I'm doing!",
        (_, _, true)     => "Fine. Be that way!",
        (_, _, _)        => "Whatever.",
    }
}
