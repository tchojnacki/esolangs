mod instruction;
mod lexer;
mod parser;
mod vm;

pub use {instruction::Instruction, lexer::tokenize, parser::parse, vm::VirtualMachine};
