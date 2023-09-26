use backend::{emitter::emit, instruction::Program, optimizer::optimize};
use frontend::{lexer::tokenize, parser::parse};

mod backend;
mod frontend;
pub mod util;

pub use self::{
    backend::{
        instruction::Instruction,
        settings::Settings,
        vm::{RuntimeError, VirtualMachine, VirtualMachineStd},
    },
    frontend::parser::ParseError,
};

pub fn compile(code: &str, settings: &Settings) -> Result<Program, ParseError> {
    let tokens = tokenize(code);
    let ast = parse(tokens)?;
    let program = emit(&ast);
    Ok(optimize(program, settings))
}
