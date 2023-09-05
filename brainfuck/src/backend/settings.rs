#[must_use]
pub struct Settings {
    pub tape_length: u32,
    pub strict: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tape_length: 30_000,
            strict: false,
        }
    }
}
