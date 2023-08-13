#[must_use]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    StartLoop,
    EndLoop,
}

pub fn tokenize(code: impl IntoIterator<Item = char>) -> impl Iterator<Item = Token> {
    use Token::*;
    code.into_iter().filter_map(|c| match c {
        '>' => Some(Right),
        '<' => Some(Left),
        '+' => Some(Increment),
        '-' => Some(Decrement),
        '.' => Some(Output),
        ',' => Some(Input),
        '[' => Some(StartLoop),
        ']' => Some(EndLoop),
        _ => None,
    })
}
