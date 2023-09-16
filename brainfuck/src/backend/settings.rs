#[must_use]
#[derive(Clone, Debug)]
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

    pub fn mut_cell(&self, cell: u8, change: i8) -> Option<u8> {
        match self.strict {
            true => cell.checked_add_signed(change),
            false => Some(cell.wrapping_add_signed(change)),
        }
    }

    pub fn mut_pointer(&self, pointer: usize, change: i32) -> Option<usize> {
        let new = pointer as i32 + change;
        if self.strict && (new < 0 || new >= self.tape_length as i32) {
            return None;
        }
        Some(new.rem_euclid(self.tape_length as i32) as usize)
    }
}
