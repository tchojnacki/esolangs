use super::Module;
use crate::indices::{FuncIdx, GlobalIdx, MemIdx, WasmIndex};

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl From<FuncIdx> for ExportDesc {
    fn from(func_idx: FuncIdx) -> Self {
        Self::Func(func_idx)
    }
}

impl From<MemIdx> for ExportDesc {
    fn from(mem_idx: MemIdx) -> Self {
        Self::Mem(mem_idx)
    }
}

impl From<GlobalIdx> for ExportDesc {
    fn from(global_idx: GlobalIdx) -> Self {
        Self::Global(global_idx)
    }
}

impl ExportDesc {
    pub(crate) fn emit_wat_inline(&self, module: &Module) -> String {
        match self {
            ExportDesc::Func(func_idx) => format!("(func {})", func_idx.id_or_index(module)),
            ExportDesc::Mem(mem_idx) => format!("(memory {})", mem_idx.id_or_index(module)),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Export {
    pub(crate) name: String,
    pub(crate) desc: ExportDesc,
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
