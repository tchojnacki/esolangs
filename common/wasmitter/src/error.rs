#[non_exhaustive]
#[derive(Debug)]
pub enum WasmError {
    ModuleMismatch,
    FuncMismatch,
    InvalidLimits { min: u32, max: u32 },
    InvalidIdentifier { id: &'static str },
    InvalidLabel { index: u32 },
}
