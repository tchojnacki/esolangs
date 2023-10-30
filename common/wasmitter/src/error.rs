use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum WasmError {
    #[error("module mismatch")]
    ModuleMismatch,
    #[error("function mismatch")]
    FuncMismatch,
    #[error("invalid limits: {min} {max}")]
    InvalidLimits { min: u32, max: u32 },
    #[error("invalid identifier: {id}")]
    InvalidIdentifier { id: &'static str },
    #[error("invalid label: {index}")]
    InvalidLabel { index: u32 },
}
