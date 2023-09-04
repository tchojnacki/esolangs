use brainfuck::{ParseError, RuntimeError};

pub trait BfError {
    fn message(&self, source: &str) -> String;
}

impl BfError for ParseError {
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

impl BfError for RuntimeError {
    fn message(&self, _: &str) -> String {
        match self {
            RuntimeError::InputError => "RuntimeError: Could not read from input.",
            RuntimeError::OutputError => "RuntimeError: Could not write to output.",
        }
        .to_owned()
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
