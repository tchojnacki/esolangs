mod const_instr;
mod instr;

pub use self::{
    const_instr::ConstInstr,
    instr::{BlockType, Instr, MemArg, Nn, Sx},
};
