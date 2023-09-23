use brainfuck::{ParseError, RuntimeError};
use colored::Colorize;

use crate::source::highlight_source;

pub trait CliError {
    fn message(&self, source: &str) -> String;
}

impl CliError for ParseError {
    fn message(&self, source: &str) -> String {
        match self {
            ParseError::UnexpectedLoopEnd(pos) => highlight_source(
                "ParseError: Unexpected loop end.",
                source,
                *pos,
                "this bracket is unmatched",
            ),
            ParseError::MissingLoopEnd => highlight_source(
                "ParseError: Missing loop end.",
                source,
                source.len(),
                "expected ], found EOF",
            ),
        }
    }
}

impl CliError for RuntimeError {
    fn message(&self, _: &str) -> String {
        match self {
            RuntimeError::InputError => "RuntimeError: Could not read from input.".to_owned(),
            RuntimeError::OutputError => "RuntimeError: Could not write to output.".to_owned(),
            RuntimeError::TapeOverflow { from, by } => format!(
                "RuntimeError: Tape address overflowed.\nAttempted to change pointer equal to {from} by {by}."
            ),
            RuntimeError::CellOverflow { at, from, by } => format!(
                "RuntimeError: Cell overflowed.\nAttempted to change a cell equal to {from} by {by} at address {at}."
            ),
        }
    }
}

pub fn show_error(message: &str) {
    eprintln!("\n{}", message.red());
}
