use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
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
