mod const_instr;
mod expr;
mod instr;

pub use self::{
    const_instr::ConstInstr,
    expr::Expr,
    instr::{BlockType, Instr, MemArg, Nn, Sx},
};
