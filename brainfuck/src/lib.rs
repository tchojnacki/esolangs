mod ast;
mod emitter;
mod instruction;
mod lexer;
mod optimizer;
mod parser;
mod util;
mod vm;

use emitter::emit;
use instruction::Program;
use lexer::tokenize;
use optimizer::optimize;
use parser::parse;

pub use {
    parser::ParseError,
    util::read_u8,
    vm::{RuntimeError, VirtualMachine},
};

pub fn compile(code: &str, optimization: bool) -> Result<Program, ParseError> {
    let tokens = tokenize(code.chars());
    let ast = parse(tokens)?;
    let program = emit(&ast);
    Ok(match optimization {
        true => optimize(program),
        false => program,
    })
}
