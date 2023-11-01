use crate::{
    indices::{FuncIdx, GlobalIdx, MemIdx},
    internal::WasmIndex,
    module::Module,
};

#[must_use]
#[derive(Debug)]
enum ExportDescKind {
    Func(FuncIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

/// Describes an export from the module.
///
/// Used as an argument to [`Module::export`].
///
/// This type can't be directly constructed. Instead, use conversions ([`ExportDesc::from`]):
/// - [`FuncIdx`]
/// - [`MemIdx`]
/// - [`GlobalIdx`]
///
/// # Examples
/// ```
/// # use wasmitter::{Module, indices::{FuncIdx, MemIdx, GlobalIdx}, types::Mut, instruction::ConstInstr};
/// # let mut module = Module::new();
/// let func_idx: FuncIdx = module.import_func("external", "func", "$func", (), ());
/// let mem_idx: MemIdx = module.memory("$mem", 1);
/// let global_idx: GlobalIdx = module.global("$global", Mut::Const, ConstInstr::I32Const(42));
///
/// module.export("func", func_idx);
/// module.export("mem", mem_idx);
/// module.export("global", global_idx);
/// # assert!(module.validate().is_none());
/// ```
#[must_use]
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

    #[must_use]
    fn emit_wat_inline(&self, module: &Module) -> String {
        match self.0 {
            ExportDescKind::Func(idx) => format!("(func {})", idx.id_or_index(module)),
            ExportDescKind::Mem(idx) => format!("(memory {})", idx.id_or_index(module)),
            ExportDescKind::Global(idx) => format!("(global {})", idx.id_or_index(module)),
        }
    }
}

#[must_use]
#[derive(Debug)]
pub(crate) struct Export {
    name: String,
    desc: ExportDesc,
}

impl Export {
    #[must_use]
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(export \"{}\" {})\n",
            " ".repeat(indent),
            self.name,
            self.desc.emit_wat_inline(module)
        )
    }
}
