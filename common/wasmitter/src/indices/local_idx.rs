use crate::{
    function::Func,
    internal::{FuncUid, WasmIndex},
    module::Module,
    text::Id,
    WasmError,
};

#[must_use]
#[derive(Debug, Clone, Copy)]
enum LocalIdxKind {
    Param(u32),
    Local(u32),
}

/// References a local (or a parameter) in a function.
///
/// Can be obtained from:
/// - [`FuncScope::add_param`](crate::function::FuncScope::add_param)
/// - [`FuncScope::add_local`](crate::function::FuncScope::add_local)
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct LocalIdx {
    func_uid: FuncUid,
    kind: LocalIdxKind,
}

impl LocalIdx {
    pub(crate) fn param(func_uid: FuncUid, index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Param(index),
            func_uid,
        }
    }

    pub(crate) fn local(func_uid: FuncUid, index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Local(index),
            func_uid,
        }
    }

    #[must_use]
    pub(crate) fn validate(&self, func: &Func) -> Option<WasmError> {
        if self.func_uid != func.uid() {
            Some(WasmError::FuncMismatch)
        } else {
            None
        }
    }
}

impl<'a> WasmIndex<'a> for LocalIdx {
    type Ctx = (&'a Module, &'a Func);

    #[must_use]
    fn resolve(&self, (module, func): (&'a Module, &'a Func)) -> u32 {
        let func_type = module.get_signature(func.type_idx());
        match self.kind {
            LocalIdxKind::Param(idx) => idx,
            LocalIdxKind::Local(idx) => func_type.params.len() as u32 + idx,
        }
    }

    fn id(&self) -> Id {
        Id::none()
    }
}
