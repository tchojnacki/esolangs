use std::fmt::{self, Display, Formatter};

#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct MemArg<const N: usize> {
    offset: u32,
    align: u32,
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
