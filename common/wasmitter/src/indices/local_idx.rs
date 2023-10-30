use crate::{
    function::Func,
    internal::{FuncUid, WasmIndex},
    module::Module,
    text::Id,
};

#[derive(Clone, Copy, Debug)]
enum LocalIdxKind {
    Param(u32),
    Local(u32),
}

#[derive(Clone, Copy, Debug)]
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
}

impl<'a> WasmIndex<'a> for LocalIdx {
    type Ctx = (&'a Module, &'a Func);

    fn resolve(&self, (module, func): (&'a Module, &'a Func)) -> u32 {
        let func_type = module.get_signature(func.type_idx);
        match self.kind {
            LocalIdxKind::Param(idx) => idx,
            LocalIdxKind::Local(idx) => func_type.params.0.len() as u32 + idx,
        }
    }

    fn id(&self) -> Id {
        Id::none()
    }

    fn belongs_to(&self, (_, func): (&'a Module, &'a Func)) -> bool {
        self.func_uid == func.uid
    }
}
