use crate::{indices::WasmIndex, internal::ModuleUid, module::Module, text::Id};

#[derive(Clone, Copy, Debug)]
enum FuncIdxKind {
    Imported(u32),
    Defined(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct FuncIdx {
    module_uid: ModuleUid,
    kind: FuncIdxKind,
    id: Id,
}

impl FuncIdx {
    pub(crate) fn import(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: FuncIdxKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            kind: FuncIdxKind::Defined(index),
            id,
        }
    }
}

impl<'a> WasmIndex<'a> for FuncIdx {
    type Ctx = &'a Module;

    fn resolve(&self, module: &'a Module) -> u32 {
        match self.kind {
            FuncIdxKind::Imported(idx) => idx,
            FuncIdxKind::Defined(idx) => module.import_count() + idx,
        }
    }

    fn id(&self) -> Id {
        self.id
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
