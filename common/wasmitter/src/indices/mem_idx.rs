use crate::{
    internal::{IndexKind, ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

#[derive(Clone, Copy, Debug)]
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

    pub(crate) fn validate(&self) -> Option<WasmError> {
        self.id.validate()
    }
}

impl<'a> WasmIndex<'a> for MemIdx {
    type Ctx = &'a Module;

    fn resolve(&self, module: &'a Module) -> u32 {
        self.kind.resolve(module.mem_import_count())
    }

    fn id(&self) -> Id {
        self.id
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
