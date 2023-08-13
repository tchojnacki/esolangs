pub type Procedure = Box<[Instruction]>;

#[must_use]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Procedure),
}
