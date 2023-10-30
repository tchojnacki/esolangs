mod function;
mod global;
mod limits;
mod memory;
mod number;
mod result;
mod value;

pub(crate) use self::{function::FuncType, global::GlobalType, memory::MemType};
pub use self::{
    global::Mut,
    limits::Limits,
    number::NumType,
    result::ResultType,
    value::{ValType, F32, F64, I32, I64},
};
