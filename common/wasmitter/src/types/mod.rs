mod function;
mod global;
mod limits;
mod memory;
mod number;
mod result;
mod value;

pub use self::{
    function::FuncType,
    global::Mut,
    limits::Limits,
    number::NumType,
    result::ResultType,
    value::{ValType, F32, F64, I32, I64},
};
pub(crate) use self::{global::GlobalType, memory::MemType};
