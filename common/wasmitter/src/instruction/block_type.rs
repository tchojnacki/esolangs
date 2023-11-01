use crate::{
    indices::TypeIdx,
    module::Module,
    types::{FuncType, ResultType, ValType},
};

#[must_use]
#[derive(Debug, Clone)]
enum BlockTypeKind {
    #[allow(dead_code)]
    Type(TypeIdx),
    Val(Option<ValType>),
}

/// Block type, which defines the types in consumes and produces.
///
/// **Note:** Currently, only [`BlockType::default`] is supported, which
/// represents a block that doesn't consume or produce any values.
///
/// # Examples
/// ```
/// # use wasmitter::{Module, Instr, instruction::BlockType};
/// # let mut module = Module::new();
/// module.func("$func", |scope| {
///     Instr::Block(BlockType::default(), vec![Instr::Br(0.into())])
/// });
/// # assert!(module.validate().is_none());
/// ```
///
/// # Specification
/// - [Control Instructions - Structure](https://webassembly.github.io/spec/core/syntax/instructions.html#control-instructions)
/// - [Control Instructions - Text Format](https://webassembly.github.io/spec/core/text/instructions.html#control-instructions)
#[must_use]
#[derive(Debug, Clone)]
pub struct BlockType(BlockTypeKind);

impl Default for BlockType {
    fn default() -> Self {
        Self(BlockTypeKind::Val(None))
    }
}

impl BlockType {
    /// Creates a block which doesn't consume or produce any values.
    ///
    /// Same as [`BlockType::default`].
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub(crate) fn emit_wat_inline(&self, module: &Module) -> String {
        let func_type = match &self.0 {
            BlockTypeKind::Type(type_idx) => module.get_signature(*type_idx).clone(),
            BlockTypeKind::Val(val_type) => FuncType {
                params: ResultType::default(),
                results: match val_type {
                    Some(val_type) => vec![val_type.clone()],
                    None => Vec::new(),
                }
                .into(),
            },
        };

        func_type.emit_wat_inline()
    }
}
