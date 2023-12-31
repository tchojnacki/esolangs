use crate::{
    internal::{ModuleUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub(crate) struct TypeIdx {
    module_uid: ModuleUid,
    index: u32,
}

impl TypeIdx {
    pub(crate) fn new(module_uid: ModuleUid, index: u32) -> Self {
        Self { module_uid, index }
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        if module.uid() != self.module_uid {
            Some(WasmError::ModuleMismatch)
        } else {
            None
        }
    }
}

impl<'a> WasmIndex<'a> for TypeIdx {
    type Ctx = &'a Module;

    #[must_use]
    fn resolve(&self, _: &'a Module) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        Id::none()
    }
}
