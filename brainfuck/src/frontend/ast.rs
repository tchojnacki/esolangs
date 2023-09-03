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
}

pub type Tree = Box<[Node]>;
