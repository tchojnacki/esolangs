use crate::WasmError;

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
