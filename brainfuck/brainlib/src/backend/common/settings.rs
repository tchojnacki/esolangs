#[must_use]
#[derive(Clone, Debug)]
pub struct Settings {
    tape_length: u32,
    strict: bool,
    debug: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tape_length: Self::DEFAULT_LENGTH,
            strict: false,
            debug: false,
        }
    }
}

impl Settings {
    pub const DEFAULT_LENGTH: u32 = 30_000;

    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn try_new(tape_length: u32, strict: bool, debug: bool) -> Option<Self> {
        match tape_length {
            3..=1_000_000_000 => Some(Self {
                tape_length,
                strict,
                debug,
            }),
            _ => None,
        }
    }

    #[must_use]
    pub const fn tape_length(&self) -> u32 {
        self.tape_length
    }

    #[must_use]
    pub const fn strict(&self) -> bool {
        self.strict
    }

    #[must_use]
    pub const fn debug(&self) -> bool {
        self.debug
    }

    pub const fn with_strict(self) -> Self {
        Self {
            strict: true,
            ..self
        }
    }

    pub const fn without_strict(self) -> Self {
        Self {
            strict: false,
            ..self
        }
    }

    pub const fn with_debug(self) -> Self {
        Self {
            debug: true,
            ..self
        }
    }

    pub const fn without_debug(self) -> Self {
        Self {
            debug: false,
            ..self
        }
    }

    #[must_use]
    pub(crate) const fn mut_cell(&self, cell: u8, change: i8) -> Option<u8> {
        match self.strict {
            true => cell.checked_add_signed(change),
            false => Some(cell.wrapping_add_signed(change)),
        }
    }

    #[must_use]
    pub(crate) const fn mut_pointer(&self, pointer: u32, change: i32) -> Option<u32> {
        let new = pointer as i32 + change;
        if self.strict && (new < 0 || new >= self.tape_length as i32) {
            return None;
        }
        Some(new.rem_euclid(self.tape_length as i32) as u32)
    }
}
