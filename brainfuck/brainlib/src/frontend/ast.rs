#[must_use]
#[derive(Debug, PartialEq)]
pub enum Node {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Tree),
    Breakpoint(usize),
}

pub type Tree = Box<[Node]>;
