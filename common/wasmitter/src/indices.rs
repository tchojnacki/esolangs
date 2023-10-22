use std::borrow::Cow;

use crate::{module::Module, types::Func, Id};

pub(crate) trait WasmIndex<'a>: Clone + Copy {
    type Ctx;

    fn resolve(&self, ctx: Self::Ctx) -> u32;
    fn id(&self) -> Id;

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
    index: u32,
}

impl TypeIdx {
    pub(crate) fn new(index: u32) -> Self {
        Self { index }
    }
}

impl WasmIndex<'_> for TypeIdx {
    type Ctx = ();

    fn resolve(&self, _: ()) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        Id::none()
    }
}

#[derive(Clone, Copy, Debug)]
enum FuncIdxKind {
    Imported(u32),
    Defined(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct FuncIdx {
    kind: FuncIdxKind,
    id: Id,
}

impl FuncIdx {
    pub(crate) fn import(index: u32, id: Id) -> Self {
        Self {
            kind: FuncIdxKind::Imported(index),
            id,
        }
    }

    pub(crate) fn define(index: u32, id: Id) -> Self {
        Self {
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
}

#[derive(Clone, Copy, Debug)]
pub struct MemIdx {
    index: u32,
    id: Id,
}

impl MemIdx {
    pub(crate) fn new(index: u32, id: Id) -> Self {
        Self { index, id }
    }
}

impl WasmIndex<'_> for MemIdx {
    type Ctx = ();

    fn resolve(&self, _: ()) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalIdx {
    index: u32,
    id: Id,
}

impl GlobalIdx {
    pub(crate) fn new(index: u32, id: Id) -> Self {
        Self { index, id }
    }
}

impl WasmIndex<'_> for GlobalIdx {
    type Ctx = ();

    fn resolve(&self, _: ()) -> u32 {
        self.index
    }

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DataIdx(pub(crate) u32);

impl WasmIndex<'_> for DataIdx {
    type Ctx = ();

    fn resolve(&self, _: ()) -> u32 {
        self.0
    }

    fn id(&self) -> Id {
        Id::none()
    }
}

#[derive(Clone, Copy, Debug)]
enum LocalIdxKind {
    Param(u32),
    Local(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct LocalIdx {
    kind: LocalIdxKind,
}

impl LocalIdx {
    pub(crate) fn param(index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Param(index),
        }
    }

    pub(crate) fn local(index: u32) -> Self {
        Self {
            kind: LocalIdxKind::Local(index),
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
}

impl From<u32> for LabelIdx {
    fn from(index: u32) -> Self {
        Self(index)
    }
}
