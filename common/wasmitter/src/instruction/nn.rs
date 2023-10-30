use std::fmt::{self, Display, Formatter};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum Nn {
    N32,
    N64,
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