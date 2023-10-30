use crate::{
    internal::{ModuleUid, WasmIndex},
    module::Module,
    text::Id,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct TypeIdx {
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
}
