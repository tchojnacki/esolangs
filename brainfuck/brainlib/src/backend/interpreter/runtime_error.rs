use thiserror::Error;

#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum RuntimeError {
    #[error("input error")]
    InputError,
    #[error("output error")]
    OutputError,
    #[error("tape overflow when changing {from} by {by}")]
    TapeOverflow { from: u32, by: i32 },
    #[error("cell overflow when changing {from} by {by} at {at}")]
    CellOverflow { at: u32, from: u8, by: i8 },
}
