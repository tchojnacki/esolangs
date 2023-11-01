use crate::{
    internal::{IndexKind, ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

/// References a single memory (imported or defined) within a module.
///
/// Can be obtained from:
/// - [`Module::import_memory`]
/// - [`Module::memory`]
///
/// # Examples
/// ```
/// # use wasmitter::{Module, indices::MemIdx};
/// # let mut module = Module::new();
/// let mem_idx: MemIdx = module.memory("$my_memory", 1);
///
/// module.export("my_memory", mem_idx);
/// # assert!(module.validate().is_none())
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct MemIdx {
    module_uid: ModuleUid,
    kind: IndexKind,
    id: Id,
}

impl MemIdx {
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
        if module.uid() != self.module_uid {
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

impl<'a> WasmIndex<'a> for MemIdx {
    type Ctx = &'a Module;

    #[must_use]
    fn resolve(&self, module: &'a Module) -> u32 {
        self.kind.resolve(module.mem_import_count())
    }

    fn id(&self) -> Id {
        self.id
    }
}
