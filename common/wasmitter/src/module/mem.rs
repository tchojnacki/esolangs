use crate::{
    indices::{MemIdx, WasmIndex},
    module::Module,
    types::Limits,
};

#[derive(Debug)]
pub(crate) struct MemType {
    pub(crate) limits: Limits,
}

#[derive(Debug)]
pub(crate) struct Mem {
    pub(crate) mem_type: MemType,
    pub(crate) mem_idx: MemIdx,
}

impl Mem {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(memory {} {} {})\n",
            " ".repeat(indent),
            self.mem_idx.id_or_comment(module),
            self.mem_type.limits.min,
            self.mem_type.limits.max
        )
    }
}
