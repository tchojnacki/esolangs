mod bytecode;
mod lexer;
mod parser;
mod util;
mod vm;

use bytecode::{generate, optimize, Program};
use lexer::tokenize;
use parser::parse;

pub use {
    parser::ParseError,
    util::read_u8,
    vm::{RuntimeError, VirtualMachine},
};

pub fn compile(code: &str, optimization: bool) -> Result<Program, ParseError> {
    let tokens = tokenize(code.chars());
    let ast = parse(tokens)?;
    let program = generate(&ast);
    Ok(match optimization {
        true => optimize(program),
        false => program,
    })
}
