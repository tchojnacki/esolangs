use super::{emitter::emit, instruction::Instruction, optimizer::optimize};
use crate::{
    frontend::{lexer::tokenize, parser::parse},
    ParseError, Settings,
};

#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program(pub(crate) Vec<Instruction>);

impl Program {
    pub fn compile(source: &str, settings: &Settings) -> Result<Self, ParseError> {
        let tokens = tokenize(source);
        let ast = parse(tokens)?;
        let program = emit(&ast);
        Ok(optimize(program, settings))
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
