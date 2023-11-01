use std::fmt::{self, Display, Formatter};

/// A numeric type, either `i32`, `i64`, `f32` or `f64`.
///
/// # Specification
/// - [Number Types - Structure](https://webassembly.github.io/spec/core/syntax/types.html#number-types)
/// - [Number Types - Text Format](https://webassembly.github.io/spec/core/text/types.html#number-types)
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumType {
    /// `i32`
    I32,

    /// `i64`
    I64,

    /// `f32`
    F32,

    /// `f64`
    F64,
}

impl NumType {
    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            NumType::I32 => "i32",
            NumType::I64 => "i64",
            NumType::F32 => "f32",
            NumType::F64 => "f64",
        }
        .into()
    }
}

impl Display for NumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.emit_wat_inline().as_str())
    }
}
