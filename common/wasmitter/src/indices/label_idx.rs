use crate::{internal::WasmIndex, text::Id, WasmError};

/// References an unnamed label within a function.
///
/// May cause [`WasmError::InvalidLabel`] during validation if the label index is out of bounds.
///
/// # Examples
/// ```
/// # use wasmitter::{Module, Instr, indices::LabelIdx, instruction::BlockType};
/// # let mut module = Module::new();
/// module.func("$func", |scope| {
///     Instr::Block(BlockType::default(), vec![Instr::Br(LabelIdx::from(0))])
/// });
/// # assert!(module.validate().is_none());
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct LabelIdx(pub(crate) u32);

impl LabelIdx {
    #[must_use]
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

    #[must_use]
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
