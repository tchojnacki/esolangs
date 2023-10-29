use crate::{indices::WasmIndex, internal::ModuleUid, module::Module, text::Id};

#[derive(Clone, Copy, Debug)]
pub struct GlobalIdx {
    module_uid: ModuleUid,
    index: u32,
    id: Id,
}

impl GlobalIdx {
    pub(crate) fn new(module_uid: ModuleUid, index: u32, id: Id) -> Self {
        Self {
            module_uid,
            index,
            id,
        }
    }
}

impl<'a> WasmIndex<'a> for GlobalIdx {
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
