use crate::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TypeIdx},
    internal::WasmIndex,
    module::Module,
    types::{GlobalType, MemType},
};

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
        global_type: GlobalType,
        global_idx: GlobalIdx,
    ) -> Self {
        Self {
            module,
            name,
            desc: ImportDesc::Global {
                global_type,
                global_idx,
            },
        }
    }

    pub(crate) fn is_func(&self) -> bool {
        matches!(self.desc, ImportDesc::Func { .. })
    }

    pub(crate) fn is_mem(&self) -> bool {
        matches!(self.desc, ImportDesc::Mem { .. })
    }

    pub(crate) fn is_global(&self) -> bool {
        matches!(self.desc, ImportDesc::Global { .. })
    }

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
