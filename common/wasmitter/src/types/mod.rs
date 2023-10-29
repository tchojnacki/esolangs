mod expr;
mod func_type;
mod limits;
mod num_type;
mod result_type;
mod val_type;

pub(crate) use self::limits::Limits;
pub use self::{
    expr::Expr,
    func_type::FuncType,
    num_type::NumType,
    result_type::ResultType,
    val_type::{ValType, F32, F64, I32, I64},
};
