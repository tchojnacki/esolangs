use std::fmt::{self, Display, Formatter};

/// Signedness specifier for instructions.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum Sx {
    /// Instruction operates on unsigned values.
    U,

    /// Instruction operates on signed values.
    S,
}

impl Display for Sx {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sx::U => "u",
                Sx::S => "s",
            }
        )
    }
}
