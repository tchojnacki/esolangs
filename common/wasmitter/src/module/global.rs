use crate::{
    indices::GlobalIdx, instruction::ConstInstr, internal::WasmIndex, module::Module,
    types::GlobalType, WasmError,
};

#[derive(Debug)]
pub(crate) struct Global {
    pub(crate) global_type: GlobalType,
    pub(crate) init: ConstInstr,
    pub(crate) global_idx: GlobalIdx,
}

impl Global {
    pub(crate) fn validate(&self) -> Option<WasmError> {
        self.global_idx.validate()
    }

    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(global {} {} {})\n",
            " ".repeat(indent),
            self.global_idx.id_or_comment(module),
            self.global_type.emit_wat_inline(),
            self.init.emit_wat_inline(),
        )
    }
}
