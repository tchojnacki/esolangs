use std::fmt::{self, Display, Formatter};

/// Defines the offset and alignment of a memory operation.
///
/// **NOTE:** Currently, only the default alignment (`offset=0 align=0`) is supported.
#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct MemArg<const N: usize> {
    offset: u32,
    align: u32,
}

impl<const N: usize> MemArg<N> {
    /// Creates a new `MemArg` with the default offset and alignment.
    ///
    /// Same as [`MemArg::default`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: usize> Display for MemArg<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.offset != 0 {
            write!(f, " offset={}", self.offset)?;
        }
        if self.align != 0 {
            write!(f, " align={}", self.align)?;
        }
        Ok(())
    }
}
