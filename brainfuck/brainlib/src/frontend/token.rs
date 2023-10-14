#[must_use]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

#[must_use]
#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum TokenKind {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    StartLoop,
    EndLoop,
    Debug,
}
