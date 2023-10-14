mod backend;
mod frontend;
pub mod util;

use crate::{
    backend::common::{emitter::emit, instruction::Program, optimizer::optimize},
    frontend::{lexer::tokenize, parser::parse},
};
pub use crate::{
    backend::{
        common::{instruction::Instruction, settings::Settings},
        interpreter,
    },
    frontend::parser::ParseError,
};

pub fn compile(code: &str, settings: &Settings) -> Result<Program, ParseError> {
    let tokens = tokenize(code);
    let ast = parse(tokens)?;
    let program = emit(&ast);
    Ok(optimize(program, settings))
}
