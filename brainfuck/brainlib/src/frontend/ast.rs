#[must_use]
#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Tree),
    Breakpoint(usize),
}

pub(crate) type Tree = Box<[Node]>;
