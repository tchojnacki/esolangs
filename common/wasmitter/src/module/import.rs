use crate::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TypeIdx},
    internal::WasmIndex,
    module::Module,
    types::{GlobalType, MemType, Mut, ValType},
    WasmError,
};

#[must_use]
#[derive(Debug)]
enum ImportDesc {
    Func {
        type_idx: TypeIdx,
        func_idx: FuncIdx,
    },
    Mem {
        mem_type: MemType,
        mem_idx: MemIdx,
    },
    Global {
        global_type: GlobalType,
        global_idx: GlobalIdx,
    },
}

impl ImportDesc {
    #[must_use]
    fn validate(&self, module: &Module) -> Option<WasmError> {
        match self {
            ImportDesc::Func { func_idx, .. } => func_idx.validate(module),
            ImportDesc::Mem { mem_type, mem_idx } =>
                mem_type.validate().or(mem_idx.validate(module)),
            ImportDesc::Global { global_idx, .. } => global_idx.validate(module),
        }
    }

    #[must_use]
    fn emit_wat_inline(&self, module: &Module) -> String {
        match self {
            ImportDesc::Func { type_idx, func_idx } => {
                let func_type = module.get_signature(*type_idx);
                let id = func_idx.id_or_comment(module);
                format!("(func {id} {})", func_type.emit_wat_inline())
            },
            ImportDesc::Mem { mem_type, mem_idx } => {
                let id = mem_idx.id_or_comment(module);
                format!("(memory {id} {})", mem_type.emit_wat_inline())
            },
            ImportDesc::Global {
                global_type,
                global_idx,
            } => {
                let id = global_idx.id_or_comment(module);
                format!("(global {id} {})", global_type.emit_wat_inline())
            },
        }
    }
}

#[must_use]
#[derive(Debug)]
pub(crate) struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

impl Import {
    pub(crate) fn func(module: String, name: String, type_idx: TypeIdx, func_idx: FuncIdx) -> Self {
        Self {
            module,
            name,
            desc: ImportDesc::Func { type_idx, func_idx },
        }
    }

    pub(crate) fn mem(module: String, name: String, mem_type: MemType, mem_idx: MemIdx) -> Self {
        Self {
            module,
            name,
            desc: ImportDesc::Mem { mem_type, mem_idx },
        }
    }

    pub(crate) fn global(
        module: String,
        name: String,
        mutability: Mut,
        val_type: ValType,
        global_idx: GlobalIdx,
    ) -> Self {
        let global_type = GlobalType {
            mutability,
            val_type,
        };
        Self {
            module,
            name,
            desc: ImportDesc::Global {
                global_type,
                global_idx,
            },
        }
    }

    #[must_use]
    pub(crate) fn is_func(&self) -> bool {
        matches!(self.desc, ImportDesc::Func { .. })
    }

    #[must_use]
    pub(crate) fn is_mem(&self) -> bool {
        matches!(self.desc, ImportDesc::Mem { .. })
    }

    #[must_use]
    pub(crate) fn is_global(&self) -> bool {
        matches!(self.desc, ImportDesc::Global { .. })
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.desc.validate(module)
    }

    #[must_use]
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(import \"{}\" \"{}\" {})\n",
            " ".repeat(indent),
            self.module,
            self.name,
            self.desc.emit_wat_inline(module)
        )
    }
}
