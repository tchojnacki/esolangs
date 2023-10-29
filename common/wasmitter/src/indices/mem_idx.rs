use crate::{indices::WasmIndex, internal::ModuleUid, module::Module, text::Id};

#[derive(Clone, Copy, Debug)]
pub struct MemIdx {
    module_uid: ModuleUid,
    index: u32,
    id: Id,
}

impl MemIdx {
    pub(crate) fn new(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            index,
            id,
            module_uid,
        }
    }
}

impl<'a> WasmIndex<'a> for MemIdx {
    type Ctx = &'a Module;

    fn resolve(&self, _: &'a Module) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        self.id
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
