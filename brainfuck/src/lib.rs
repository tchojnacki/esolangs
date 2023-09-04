use backend::{emitter::emit, instruction::Program, optimizer::optimize};
use frontend::{lexer::tokenize, parser::parse};

mod backend;
mod frontend;
pub mod util;

pub use {
    backend::vm::{RuntimeError, VirtualMachine},
    frontend::parser::ParseError,
};

pub fn compile(code: &str, optimization: bool) -> Result<Program, ParseError> {
    let tokens = tokenize(code);
    let ast = parse(tokens)?;
    let program = emit(&ast);
    Ok(match optimization {
        true => optimize(program),
        false => program,
    })
}
