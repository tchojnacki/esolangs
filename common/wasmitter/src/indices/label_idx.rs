use crate::{internal::WasmIndex, text::Id, WasmError};

#[derive(Debug, Clone, Copy)]
pub struct LabelIdx(pub(crate) u32);

impl LabelIdx {
    pub(crate) fn validate(&self, blocks: usize) -> Option<WasmError> {
        if self.0 as usize >= blocks {
            Some(WasmError::InvalidLabel { index: self.0 })
        } else {
            None
        }
    }
}

impl<'a> WasmIndex<'a> for LabelIdx {
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
