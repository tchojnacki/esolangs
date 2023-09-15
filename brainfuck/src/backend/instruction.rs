#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    MutPointer(i32),
    MutCell(i8),
    SetCell(u8),
    JumpRightZ(u32),
    JumpLeftNz(u32),
    Input,
    Output,
}

impl Instruction {
    pub fn unwrap_mut_pointer(&self) -> i32 {
        match self {
            Instruction::MutPointer(value) => *value,
            _ => panic!(),
        }
    }

    pub fn unwrap_mut_cell(&self) -> i8 {
        match self {
            Instruction::MutCell(value) => *value,
            _ => panic!(),
        }
    }
}

pub type Program = Vec<Instruction>;
