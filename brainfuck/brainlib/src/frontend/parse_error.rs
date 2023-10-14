use thiserror::Error;

/// Error which occured during the parsing of source code, caused by a syntax error.
///
/// This can occur during [compilation](crate::Program::compile).
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum ParseError {
    /// The loop end character `]` was found at `end_pos`, but it was not preceded by a loop start character `[`.
    #[error("unexpected loop end at position {end_pos}")]
    UnexpectedLoopEnd {
        /// The position of the unexpected loop end character `]`.
        end_pos: usize,
    },

    /// The end of the source code was reached, without closing a loop started at `start_pos` with the loop end character `]`.
    #[error("missing loop end for the loop started at position {start_pos}")]
    MissingLoopEnd {
        /// The position of the loop start character `[` which was not closed.        
        start_pos: usize,
    },
}
