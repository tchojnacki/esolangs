use std::fmt::{self, Display, Formatter};

use crate::types::{ValType, F32, F64, I32, I64};

/// Width (32 or 64 bits) specifier for instructions.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum Nn {
    /// Instruction operates on 32-bit values ([`I32`] or [`F32`]).
    N32,

    /// Instruction operates on 64-bit values ([`I64`] or [`F64`]).
    N64,
}

impl Nn {
    /// Returns the number of bits of this specifier.
    #[must_use]
    pub fn bit_width(&self) -> usize {
        match self {
            Self::N32 => 32,
            Self::N64 => 64,
        }
    }

    /// Returns the integer type corresponding to this specifier ([`I32`] or [`I64`]).
    pub fn integer_type(&self) -> ValType {
        match self {
            Self::N32 => I32,
            Self::N64 => I64,
        }
    }

    /// Returns the float type corresponding to this specifier ([`F32`] or [`F64`]).
    pub fn float_type(&self) -> ValType {
        match self {
            Self::N32 => F32,
            Self::N64 => F64,
        }
    }
}

impl Display for Nn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Nn::N32 => "32",
                Nn::N64 => "64",
            }
        )
    }
}
