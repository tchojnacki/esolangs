use crate::{
    indices::{GlobalIdx, WasmIndex},
    instructions::ConstInstr,
    module::Module,
    types::ValType,
};

#[derive(Debug)]
pub(crate) struct GlobalType {
    pub(crate) mutability: Mutability,
    pub(crate) val_type: ValType,
}

impl GlobalType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        format!(
            "({} {})",
            self.mutability.emit_wat_inline(),
            self.val_type.emit_wat_inline()
        )
    }
}

#[derive(Debug)]
pub enum Mutability {
    Mut,
    Const,
}

impl Mutability {
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            Mutability::Mut => "mut",
            Mutability::Const => "const",
        }
        .into()
    }
}

#[derive(Debug)]
pub(crate) struct Global {
    pub(crate) global_type: GlobalType,
    pub(crate) init: ConstInstr,
    pub(crate) global_idx: GlobalIdx,
}

impl Global {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        let tab = " ".repeat(indent);
        format!(
            "{tab}(global {} {}\n{}{tab})\n",
            self.global_idx.id_or_comment(module),
            self.global_type.emit_wat_inline(),
            self.init.emit_wat_block(indent + 2),
        )
    }
}
