use crate::{indices::WasmIndex, internal::ModuleUid, module::Module, text::Id};

#[derive(Clone, Copy, Debug)]
pub struct TypeIdx {
    module_uid: ModuleUid,
    index: u32,
}

impl TypeIdx {
    pub(crate) fn new(module_uid: ModuleUid, index: u32) -> Self {
        Self { module_uid, index }
    }
}

impl<'a> WasmIndex<'a> for TypeIdx {
    type Ctx = &'a Module;

    fn resolve(&self, _: &'a Module) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        Id::none()
    }

    fn belongs_to(&self, module: &'a Module) -> bool {
        self.module_uid == module.uid
    }
}
