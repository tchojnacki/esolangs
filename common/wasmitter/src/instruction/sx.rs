use std::fmt::{self, Display, Formatter};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum Sx {
    U,
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
