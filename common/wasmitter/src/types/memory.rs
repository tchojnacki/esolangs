use crate::{types::Limits, WasmError};

#[must_use]
#[derive(Debug)]
pub(crate) struct MemType {
    limits: Limits,
}

impl MemType {
    #[must_use]
    pub(crate) fn validate(&self) -> Option<WasmError> {
        self.limits.validate()
    }

    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self.limits.max {
            Some(max) => format!("{} {}", self.limits.min, max),
            None => self.limits.min.to_string(),
        }
    }
}

impl From<Limits> for MemType {
    fn from(limits: Limits) -> Self {
        Self { limits }
    }
}
