use crate::types::NumType;

/// A value type, classifies values that WebAssembly code can compute with.
///
/// **NOTE:** Currently, only numeric types are supported.
///
/// # Specification
/// - [Value Types - Structure](https://webassembly.github.io/spec/core/syntax/types.html#value-types)
/// - [Value Types - Text Format](https://webassembly.github.io/spec/core/text/types.html#value-types)
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    /// A numeric value type, see [`NumType`].
    Num(NumType),
}

impl ValType {
    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            ValType::Num(num) => num.emit_wat_inline(),
        }
    }
}

/// A shorthand for `ValType::Num(NumType::I32)`.
/// ```
/// use wasmitter::types::{NumType, ValType, I32};
/// assert_eq!(I32, ValType::Num(NumType::I32));
/// ```
pub const I32: ValType = ValType::Num(NumType::I32);

/// A shorthand for `ValType::Num(NumType::I64)`.
/// ```
/// use wasmitter::types::{NumType, ValType, I64};
/// assert_eq!(I64, ValType::Num(NumType::I64));
/// ```
pub const I64: ValType = ValType::Num(NumType::I64);

/// A shorthand for `ValType::Num(NumType::F32)`.
/// ```
/// use wasmitter::types::{NumType, ValType, F32};
/// assert_eq!(F32, ValType::Num(NumType::F32));
/// ```
pub const F32: ValType = ValType::Num(NumType::F32);

/// A shorthand for `ValType::Num(NumType::F64)`.
/// ```
/// use wasmitter::types::{NumType, ValType, F64};
/// assert_eq!(F64, ValType::Num(NumType::F64));
/// ```
pub const F64: ValType = ValType::Num(NumType::F64);
