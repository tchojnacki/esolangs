#[must_use]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

#[must_use]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    StartLoop,
    EndLoop,
}
