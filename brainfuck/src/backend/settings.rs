#[must_use]
pub struct Settings {
    tape_length: u32,
    strict: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tape_length: Self::DEFAULT_LENGTH,
            strict: false,
        }
    }
}

impl Settings {
    pub const DEFAULT_LENGTH: u32 = 30_000;

    #[must_use]
    pub const fn try_new(tape_length: u32, strict: bool) -> Option<Self> {
        match tape_length {
            3..=1_000_000_000 => Some(Self {
                tape_length,
                strict,
            }),
            _ => None,
        }
    }

    pub const fn default_strict() -> Self {
        Self {
            tape_length: Self::DEFAULT_LENGTH,
            strict: true,
        }
    }

    #[must_use]
    pub const fn tape_length(&self) -> usize {
        self.tape_length as usize
    }

    #[must_use]
    pub const fn strict(&self) -> bool {
        self.strict
    }
}
