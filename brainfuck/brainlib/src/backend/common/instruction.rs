use std::fmt::{self, Display, Formatter};

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
    Breakpoint(u32),
}

impl Instruction {
    #[must_use]
    pub(crate) fn unwrap_mut_pointer(&self) -> i32 {
        match self {
            Instruction::MutPointer(value) => *value,
            _ => panic!(),
        }
    }

    #[must_use]
    pub(crate) fn unwrap_mut_cell(&self) -> i8 {
        match self {
            Instruction::MutCell(value) => *value,
            _ => panic!(),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MutPointer(1) => write!(f, ">"),
            Self::MutPointer(-1) => write!(f, "<"),
            Self::MutCell(1) => write!(f, "+"),
            Self::MutCell(-1) => write!(f, "-"),
            Self::Output => write!(f, "."),
            Self::Input => write!(f, ","),
            Self::JumpRightZ(_) => write!(f, "["),
            Self::JumpLeftNz(_) => write!(f, "]"),
            Self::Breakpoint(_) => write!(f, "#"),
            _ => write!(f, "¿"),
        }
    }
}
