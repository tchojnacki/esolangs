use crate::WasmError;

/// A pair of minimum and an optional maximum value.
///
/// Used to specify bounds for [`Module::memory`](crate::Module::memory).
///
/// Can be converted to from a single `u32` or a pair of `u32`s.
///
/// Can return [`WasmError::InvalidLimits`] during validation if the minimum is greater than the maximum.
///
/// # Examples
/// ```
/// # use wasmitter::types::Limits;
/// let bounded_limit = Limits::from((3, 8));
/// let unbounded_limit = Limits::from(5);
/// ```
///
/// # Specification
/// - [Limits - Structure](https://webassembly.github.io/spec/core/syntax/types.html#limits)
/// - [Limits - Text Format](https://webassembly.github.io/spec/core/text/types.html#limits)
#[must_use]
#[derive(Debug)]
pub struct Limits {
    pub(crate) min: u32,
    pub(crate) max: Option<u32>,
}

impl Limits {
    #[must_use]
    pub(crate) fn validate(&self) -> Option<WasmError> {
        let min = self.min;
        let max = self.max?;

        if min <= max {
            None
        } else {
            Some(WasmError::InvalidLimits { min, max })
        }
    }
}

impl From<u32> for Limits {
    fn from(min: u32) -> Self {
        Self { min, max: None }
    }
}

impl From<(u32, u32)> for Limits {
    fn from((min, max): (u32, u32)) -> Self {
        Self {
            min,
            max: Some(max),
        }
    }
}
