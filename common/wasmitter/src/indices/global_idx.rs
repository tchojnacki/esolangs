use crate::{
    internal::{ModuleUid, WasmIndex},
    module::Module,
    text::Id,
};

#[derive(Clone, Copy, Debug)]
enum GlobalIdxKind {
    Imported(u32),
    Defined(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalIdx {
    module_uid: ModuleUid,
    kind: GlobalIdxKind,
    id: Id,
}

impl GlobalIdx {
    pub(crate) fn import(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: GlobalIdxKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: GlobalIdxKind::Defined(index),
            id,
        }
    }
}

impl<'a> WasmIndex<'a> for GlobalIdx {
    type Ctx = &'a Module;

    fn resolve(&self, module: &'a Module) -> u32 {
        match self.kind {
            GlobalIdxKind::Imported(idx) => idx,
            GlobalIdxKind::Defined(idx) => module.global_import_count() + idx,
        }
    }

    fn id(&self) -> Id {
        self.id
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
