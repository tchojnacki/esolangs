use crate::{internal::WasmIndex, text::Id};

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
