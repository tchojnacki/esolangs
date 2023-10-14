mod emitter;
mod instruction;
mod optimizer;
mod program;
mod settings;

pub(crate) use self::{emitter::emit, optimizer::optimize};
pub use self::{instruction::Instruction, program::Program, settings::Settings};
