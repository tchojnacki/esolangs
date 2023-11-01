use thiserror::Error;

/// An error occurring during module validation.
///
/// This is non-exhaustive, since additional validation may be added in the future.
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum WasmError {
    /// An index created by another module was used.
    ///
    /// [`FuncIdx`](crate::indices::FuncIdx), [`GlobalIdx`](crate::indices::GlobalIdx)
    /// and [`MemIdx`](crate::indices::MemIdx) can only be used by the module which created them.
    #[error("module mismatch")]
    ModuleMismatch,

    /// An index created by another function was used.
    ///
    /// [`LocalIdx`](crate::indices::LocalIdx) can only be used by the function which created it.
    #[error("function mismatch")]
    FuncMismatch,

    /// The [`Limits`](crate::types::Limits) are invalid, i.e. `min > max`.
    #[error("invalid limits: {min} {max}")]
    InvalidLimits {
        /// Lower bound of the invalid limits.
        min: u32,
        /// Upper bound of the invalid limits.
        max: u32,
    },

    /// The [`Id`](crate::text::Id) is invalid.
    ///
    /// > Symbolic identifiers start with `$`, followed by any sequence of printable ASCII
    /// > characters that does not contain a space, quotation mark, comma, semicolon, or bracket [^1].
    ///
    /// [^1]: [Identifiers - Text Format](https://webassembly.github.io/spec/core/text/values.html#text-id)
    #[error("invalid identifier: {id}")]
    InvalidIdentifier {
        /// The invalid identifier.
        id: &'static str,
    },

    /// A [`LabelIdx`](crate::indices::LabelIdx) does not refer to any block.
    #[error("invalid label: {index}")]
    InvalidLabel {
        /// The index of the invalid label.
        index: u32,
    },
}
