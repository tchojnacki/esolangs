#[must_use]
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    MutPointer(i32),
    MutCell(i8),
    SetCell(u8),
    RelJumpRightZero(u32),
    RelJumpLeftNotZero(u32),
    Input,
    Output,
}

pub type Program = Vec<Instruction>;
