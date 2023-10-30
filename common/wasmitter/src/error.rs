#[derive(Debug)]
pub enum WasmError {
    ModuleMismatch,
    FuncMismatch,
    InvalidLimits,
}
