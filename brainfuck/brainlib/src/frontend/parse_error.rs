use thiserror::Error;

#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum ParseError {
    #[error("unexpected loop end at position {0}")]
    UnexpectedLoopEnd(usize),
    #[error("missing loop end")]
    MissingLoopEnd,
}
