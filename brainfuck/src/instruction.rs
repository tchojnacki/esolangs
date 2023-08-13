pub type Procedure = Box<[Instruction]>;

#[must_use]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Procedure),
}
