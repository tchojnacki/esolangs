mod error;
pub mod function;
pub mod indices;
pub mod instruction;
mod internal;
pub mod module;
pub mod text;
pub mod types;

pub use self::{error::WasmError, instruction::Instr, module::Module};
