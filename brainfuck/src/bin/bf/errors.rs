use brainfuck::{ParseError, RuntimeError};

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

fn highlight_source(header: &str, source: &str, pos: usize, message: &str) -> String {
    let (line, col) = line_col(source, pos);
    let padded = &format!("     {source}      ")[pos..pos + 11];
    format!("{header}\n  | {padded}\n  |      ^ {message}\n  |        at {line}:{col}")
}

fn line_col(source: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for c in source[..pos].chars() {
        if c == '\n' {
            line += 1;
            col = 1;
        } else if !c.is_control() {
            col += 1;
        }
    }
    (line, col)
}
