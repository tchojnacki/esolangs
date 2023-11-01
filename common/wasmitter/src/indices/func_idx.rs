use crate::{
    internal::{IndexKind, ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

/// References a single function (imported or defined) within a module.
///
/// Can be obtained from:
/// - [`Module::import_func`]
/// - [`Module::func`]
///
/// # Examples
/// ```
/// # use wasmitter::{Module, indices::FuncIdx, types::I32};
/// # let mut module = Module::new();
/// let func_idx: FuncIdx = module.import_func(
///     "wasi_unstable",
///     "fd_read",
///     "$fd_read",
///     (I32, I32, I32, I32),
///     I32,
/// );
///
/// module.export("fd_read", func_idx);
/// # assert!(module.validate().is_none());
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct FuncIdx {
    module_uid: ModuleUid,
    kind: IndexKind,
    id: Id,
}

impl FuncIdx {
    pub(crate) fn import(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: IndexKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: IndexKind::Defined(index),
            id,
        }
    }

    #[must_use]
    fn validate_ownership(&self, module: &Module) -> Option<WasmError> {
        if self.module_uid != module.uid() {
            Some(WasmError::ModuleMismatch)
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.validate_ownership(module).or(self.id.validate())
    }
}

impl<'a> WasmIndex<'a> for FuncIdx {
    type Ctx = &'a Module;

    #[must_use]
    fn resolve(&self, module: &'a Module) -> u32 {
        self.kind.resolve(module.func_import_count())
    }

    fn id(&self) -> Id {
        self.id
    }
}
