use thiserror::Error;

/// Error which occurs during the interpretation of the code.
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum RuntimeError {
    /// The input could not be read from for an unknown reason.
    #[error("input error")]
    InputError,

    /// The output could not be written to for an unknown reason.
    #[error("output error")]
    OutputError,

    /// The tape pointer overflowed in `strict` mode.
    #[error("tape overflow when changing {from} by {by}")]
    TapeOverflow {
        /// The position of the pointer before the overflow.
        from: u32,
        /// The amount by which the pointer was supposed to be changed.
        by: i32,
    },

    /// A cell overflowed in `strict` mode.
    #[error("cell overflow when changing {from} by {by} at {at}")]
    CellOverflow {
        /// The index of the changed cell.
        at: u32,
        /// The value of the cell before the overflow.
        from: u8,
        /// The amount by which the cell was supposed to be changed.
        by: i8,
    },
}
