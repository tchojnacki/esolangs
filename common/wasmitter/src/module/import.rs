use crate::{
    indices::{FuncIdx, TypeIdx, WasmIndex},
    module::{GlobalType, MemType, Module},
};

#[derive(Debug)]
pub(crate) enum ImportDesc {
    Func {
        type_idx: TypeIdx,
        func_idx: FuncIdx,
    },
    Mem(MemType),
    Global(GlobalType),
}

impl ImportDesc {
    pub(crate) fn emit_wat_inline(&self, module: &Module) -> String {
        match self {
            ImportDesc::Func { type_idx, func_idx } => {
                let func_type = module.get_signature(*type_idx);
                let alias = func_idx.id_or_comment(module);
                format!("(func {} {})", alias, func_type.emit_wat_inline())
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

impl Import {
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
