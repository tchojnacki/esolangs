use std::fmt::{self, Display, Formatter};

/// Bytecode instruction, this doesn't have to directly correspond to source characters.
///
/// A list of instructions is stored inside a [`Program`](crate::Program).
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// Change the pointer by the given `i32` amount.
    ///
    /// Positive values move the pointer to the right, negative values move it to the left.
    ///
    /// Character representation:
    /// - `>` -> `MutPointer(1)`
    /// - `<` -> `MutPointer(-1)`
    MutPointer(i32),

    /// Change the value of the cell under the pointer by the given `i8` amount.
    ///
    /// Positive values increase the value, negative values decrease it.
    ///
    /// Character representation:
    /// - `+` -> `MutCell(1)`
    /// - `-` -> `MutCell(-1)`
    MutCell(i8),

    /// Set the value of the cell under the pointer to the given `u8` value.
    ///
    /// No possible representation in source code, this is only created during optimization.
    SetCell(u8),

    /// Increase the pointer by the given `u32` amount if the cell under the pointer is zero.
    ///
    /// Character representation:
    /// - `[` -> `JumpRightZ(offset)` (where `offset` is the distance to the matching `]`)
    JumpRightZ(u32),

    /// Decrease the pointer by the given `u32` amount if the cell under the pointer is **not** zero.
    ///
    /// Character representation:
    /// - `]` -> `JumpLeftNz(offset)` (where `offset` is the distance to the matching `[`)
    JumpLeftNz(u32),

    /// Take a single byte of input and store it in the cell under the pointer.
    ///
    /// Character representation:
    /// - `,` -> `Input`
    Input,

    /// Output the value of the cell under the pointer as a ASCII character.
    ///
    /// Character representation:
    /// - `.` -> `Output`
    Output,

    /// A breakpoint, this instruction is ignored outside of debug mode.
    ///
    /// Character representation:
    /// - `#` -> `Breakpoint(position)` (where `position` is the position of the `#` in the source code)
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
    /// Only the instructions that have a direct representation in source code are displayed.
    ///
    /// Other instructions are displayed as `¿`, even though they are valid instructions.
    ///
    /// # Examples
    /// ```
    /// # use brainlib::Instruction;
    /// assert_eq!(Instruction::MutCell(1).to_string(), "+");
    /// assert_eq!(Instruction::MutCell(-1).to_string(), "-");
    /// assert_eq!(Instruction::MutCell(10).to_string(), "¿");
    /// ```
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
