use std::fmt::{self, Display};

use crate::{
    backend::common::{emit, optimize, Instruction},
    frontend::{parse, tokenize},
    ParseError, Settings,
};

#[must_use]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Program(pub(crate) Vec<Instruction>);

impl Program {
    pub fn compile(source: &str, settings: &Settings) -> Result<Self, ParseError> {
        let tokens = tokenize(source);
        let ast = parse(tokens)?;
        let program = emit(&ast);
        Ok(optimize(program, settings))
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn code(&self) -> &[Instruction] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruction in &self.0 {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}
