use crate::{
    internal::{IndexKind, ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

/// References a single global (imported or defined) within a module.
///
/// Can be obtained from:
/// - [`Module::import_global`]
/// - [`Module::global`]
///
/// # Examples
/// ```
/// # use wasmitter::{Module, indices::GlobalIdx, types::Mut, instruction::ConstInstr};
/// # let mut module = Module::new();
/// let global_idx: GlobalIdx = module.global("$my_global", Mut::Const, ConstInstr::I32Const(42));
///
/// module.export("my_global", global_idx);
/// # assert!(module.validate().is_none());
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct GlobalIdx {
    module_uid: ModuleUid,
    kind: IndexKind,
    id: Id,
}

impl GlobalIdx {
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

impl<'a> WasmIndex<'a> for GlobalIdx {
    type Ctx = &'a Module;

    #[must_use]
    fn resolve(&self, module: &'a Module) -> u32 {
        self.kind.resolve(module.global_import_count())
    }

    fn id(&self) -> Id {
        self.id
    }
}
