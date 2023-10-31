/// Conventions used for compilation and interpretation.
///
/// This includes:
/// - **tape length** - how many data cells are available
/// - **strictness** - should an overflow in a data cell or the pointer be treated as an error
/// - **debugging** - should the breakpoints be enabled or ignored
///
/// # Examples
/// ```
/// # use brainlib::Settings;
/// let settings = Settings::try_new(1000, true, false).ok_or("invalid settings")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Default tape length, conventionally equal to 30 000.
    pub const DEFAULT_LENGTH: u32 = 30_000;

    /// Same as [`Settings::default`], returns the default settings.
    ///
    /// Here, tape length is 30 000, strictness is disabled and debugging is disabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates new settings with given parameters.
    ///
    /// `tape_length` must be between 3 and 1 billion (inclusive).
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

    /// Returns the set tape length.
    #[must_use]
    pub const fn tape_length(&self) -> u32 {
        self.tape_length
    }

    /// Returns the set strictness.
    #[must_use]
    pub const fn strict(&self) -> bool {
        self.strict
    }

    /// Returns the set debugging.
    #[must_use]
    pub const fn debug(&self) -> bool {
        self.debug
    }

    /// Returns the [`Settings`] with same parameters, but with strictness enabled.
    pub const fn with_strict(self) -> Self {
        Self {
            strict: true,
            ..self
        }
    }

    /// Returns the [`Settings`] with same parameters, but with strictness disabled.
    pub const fn without_strict(self) -> Self {
        Self {
            strict: false,
            ..self
        }
    }

    /// Returns the [`Settings`] with same parameters, but with debugging enabled.
    pub const fn with_debug(self) -> Self {
        Self {
            debug: true,
            ..self
        }
    }

    /// Returns the [`Settings`] with same parameters, but with debugging disabled.
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
