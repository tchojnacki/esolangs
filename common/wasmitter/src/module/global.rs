use crate::{
    indices::GlobalIdx,
    instruction::ConstInstr,
    internal::WasmIndex,
    module::Module,
    types::{GlobalType, Mut},
    WasmError,
};

#[must_use]
#[derive(Debug)]
pub(crate) struct Global {
    global_type: GlobalType,
    init: ConstInstr,
    global_idx: GlobalIdx,
}

impl Global {
    pub(crate) fn new(mutability: Mut, init: ConstInstr, global_idx: GlobalIdx) -> Self {
        let val_type = init.return_type();
        Self {
            global_type: GlobalType {
                mutability,
                val_type,
            },
            init,
            global_idx,
        }
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.global_idx.validate(module)
    }

    #[must_use]
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
