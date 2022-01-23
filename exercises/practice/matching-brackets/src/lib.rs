pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::with_capacity(string.len() / 2);
    for chr in string.chars() {
        let prev_char = stack.last();
        match (prev_char, chr) {
            (_, c) if !"{}[]()".contains(c) => (),
            (_, '{') | (_, '(') | (_, '[') => stack.push(chr),
            (Some('{'), '}') | (Some('('), ')') | (Some('['), ']') => { stack.pop(); () },
            (_, '}' | ')' | ']') => return false,
            _ => (),
        };
    }
    stack.is_empty()
}
