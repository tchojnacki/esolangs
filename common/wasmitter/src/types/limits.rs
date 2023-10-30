use crate::WasmError;

#[derive(Debug)]
pub struct Limits {
    pub(crate) min: u32,
    pub(crate) max: Option<u32>,
}

impl Limits {
    pub(crate) fn validate(&self) -> Option<WasmError> {
        match self.max {
            Some(max) if max < self.min => Some(WasmError::InvalidLimits),
            _ => None,
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
