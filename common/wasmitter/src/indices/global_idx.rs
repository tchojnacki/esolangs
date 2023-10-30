use crate::{
    internal::{IndexKind, ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

#[derive(Clone, Copy, Debug)]
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

    fn validate_ownership(&self, module: &Module) -> Option<WasmError> {
        if self.module_uid != module.uid() {
            Some(WasmError::ModuleMismatch)
        } else {
            None
        }
    }

    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.validate_ownership(module).or(self.id.validate())
    }
}

impl<'a> WasmIndex<'a> for GlobalIdx {
    type Ctx = &'a Module;

    fn resolve(&self, module: &'a Module) -> u32 {
        self.kind.resolve(module.global_import_count())
    }

    fn id(&self) -> Id {
        self.id
    }
}
