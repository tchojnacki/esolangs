use crate::{indices::MemIdx, internal::WasmIndex, module::Module, types::MemType, WasmError};

#[must_use]
#[derive(Debug)]
pub(crate) struct Mem {
    mem_type: MemType,
    mem_idx: MemIdx,
}

impl Mem {
    pub(crate) fn new(mem_type: MemType, mem_idx: MemIdx) -> Self {
        Self { mem_type, mem_idx }
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.mem_type.validate().or(self.mem_idx.validate(module))
    }

    #[must_use]
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(memory {} {})\n",
            " ".repeat(indent),
            self.mem_idx.id_or_comment(module),
            self.mem_type.emit_wat_inline()
        )
    }
}
