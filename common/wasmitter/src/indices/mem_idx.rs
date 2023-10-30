use crate::{
    internal::{ModuleUid, WasmIndex},
    module::Module,
    text::Id,
};

#[derive(Clone, Copy, Debug)]
enum MemIdxKind {
    Imported(u32),
    Defined(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct MemIdx {
    module_uid: ModuleUid,
    kind: MemIdxKind,
    id: Id,
}

impl MemIdx {
    pub(crate) fn import(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: MemIdxKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: MemIdxKind::Defined(index),
            id,
        }
    }
}

impl<'a> WasmIndex<'a> for MemIdx {
    type Ctx = &'a Module;

    fn resolve(&self, module: &'a Module) -> u32 {
        match self.kind {
            MemIdxKind::Imported(idx) => idx,
            MemIdxKind::Defined(idx) => module.mem_import_count() + idx,
        }
    }

    fn id(&self) -> Id {
        self.id
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
