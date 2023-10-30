use crate::{
    indices::{FuncIdx, GlobalIdx, MemIdx},
    internal::WasmIndex,
    module::Module,
};

#[derive(Debug)]
enum ExportDescKind {
    Func(FuncIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

#[derive(Debug)]
pub struct ExportDesc(ExportDescKind);

impl From<FuncIdx> for ExportDesc {
    fn from(func_idx: FuncIdx) -> Self {
        Self(ExportDescKind::Func(func_idx))
    }
}

impl From<MemIdx> for ExportDesc {
    fn from(mem_idx: MemIdx) -> Self {
        Self(ExportDescKind::Mem(mem_idx))
    }
}

impl From<GlobalIdx> for ExportDesc {
    fn from(global_idx: GlobalIdx) -> Self {
        Self(ExportDescKind::Global(global_idx))
    }
}

impl ExportDesc {
    pub(crate) fn into_export(self, name: String) -> Export {
        Export { name, desc: self }
    }

    fn emit_wat_inline(&self, module: &Module) -> String {
        match self.0 {
            ExportDescKind::Func(idx) => format!("(func {})", idx.id_or_index(module)),
            ExportDescKind::Mem(idx) => format!("(memory {})", idx.id_or_index(module)),
            ExportDescKind::Global(idx) => format!("(global {})", idx.id_or_index(module)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Export {
    name: String,
    desc: ExportDesc,
}

impl Export {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(export \"{}\" {})\n",
            " ".repeat(indent),
            self.name,
            self.desc.emit_wat_inline(module)
        )
    }
}
