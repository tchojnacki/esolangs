use std::borrow::Cow;

use crate::{module::Module, types::Func, FuncId, Id, ModuleId};

pub(crate) trait WasmIndex<'a>: Clone + Copy {
    type Ctx;

    fn resolve(&self, ctx: Self::Ctx) -> u32;
    fn id(&self) -> Id;
    fn belongs_to(&self, ctx: Self::Ctx) -> bool;

    fn id_or_comment(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => format!("(;{};)", self.resolve(ctx)).into(),
        }
    }

    fn id_or_index(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => self.resolve(ctx).to_string().into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TypeIdx {
    module_id: ModuleId,
    index: u32,
}

impl TypeIdx {
    pub(crate) fn new(module_id: ModuleId, index: u32) -> Self {
        Self { module_id, index }
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
        self.module_id == module.id
    }
}

#[derive(Clone, Copy, Debug)]
enum FuncIdxKind {
    Imported(u32),
    Defined(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct FuncIdx {
    module_id: ModuleId,
    kind: FuncIdxKind,
    id: Id,
}

impl FuncIdx {
    pub(crate) fn import(module_id: ModuleId, index: u32, id: Id) -> Self {
        Self {
            module_id,
            kind: FuncIdxKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(module_id: ModuleId, index: u32, id: Id) -> Self {
        Self {
            module_id,
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
        self.module_id == module.id
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemIdx {
    module_id: ModuleId,
    index: u32,
    id: Id,
}

impl MemIdx {
    pub(crate) fn new(module_id: ModuleId, index: u32, id: Id) -> Self {
        Self {
            index,
            id,
            module_id,
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
        self.module_id == module.id
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalIdx {
    module_id: ModuleId,
    index: u32,
    id: Id,
}

impl GlobalIdx {
    pub(crate) fn new(module_id: ModuleId, index: u32, id: Id) -> Self {
        Self {
            module_id,
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
        self.module_id == module.id
    }
}

#[derive(Clone, Copy, Debug)]
enum LocalIdxKind {
    Param(u32),
    Local(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct LocalIdx {
    func_id: FuncId,
    kind: LocalIdxKind,
}

impl LocalIdx {
    pub(crate) fn param(func_id: FuncId, index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Param(index),
            func_id,
        }
    }

    pub(crate) fn local(func_id: FuncId, index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Local(index),
            func_id,
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
        self.func_id == func.id
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LabelIdx(pub(crate) u32);

impl WasmIndex<'_> for LabelIdx {
    type Ctx = ();

    fn resolve(&self, _: ()) -> u32 {
        self.0
    }

    fn id(&self) -> Id {
        Id::none()
    }

    fn belongs_to(&self, _: ()) -> bool {
        true
    }
}

impl From<u32> for LabelIdx {
    fn from(index: u32) -> Self {
        Self(index)
    }
}
