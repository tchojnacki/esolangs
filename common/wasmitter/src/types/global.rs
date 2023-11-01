use crate::types::ValType;

/// Mutability of a global (`mut`), either `const` or `var`.
///
/// # Specification
/// - [Global Types - Structure](https://webassembly.github.io/spec/core/syntax/types.html#global-types)
/// - [Global Types - Text Format](https://webassembly.github.io/spec/core/text/types.html#global-types)
#[must_use]
#[derive(Debug)]
pub enum Mut {
    /// A global which can't be changed after initialization.
    Const,

    /// A global which can be changed after initialization.
    Var,
}

#[must_use]
#[derive(Debug)]
pub(crate) struct GlobalType {
    pub(crate) mutability: Mut,
    pub(crate) val_type: ValType,
}

impl GlobalType {
    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self.mutability {
            Mut::Const => self.val_type.emit_wat_inline(),
            Mut::Var => format!("(mut {})", self.val_type.emit_wat_inline()),
        }
    }
}
