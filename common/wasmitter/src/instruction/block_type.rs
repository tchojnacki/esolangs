use crate::{
    indices::TypeIdx,
    module::Module,
    types::{FuncType, ResultType, ValType},
};

#[derive(Debug, Clone)]
enum BlockTypeKind {
    #[allow(dead_code)]
    Type(TypeIdx),
    Val(Option<ValType>),
}

#[derive(Debug, Clone)]
pub struct BlockType(BlockTypeKind);

impl Default for BlockType {
    fn default() -> Self {
        Self(BlockTypeKind::Val(None))
    }
}

impl BlockType {
    pub fn new() -> Self {
        Self::default()
    }

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
