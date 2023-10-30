mod func_idx;
mod global_idx;
mod label_idx;
mod local_idx;
mod mem_idx;
mod type_idx;

pub use self::{
    func_idx::FuncIdx, global_idx::GlobalIdx, label_idx::LabelIdx, local_idx::LocalIdx,
    mem_idx::MemIdx, type_idx::TypeIdx,
};
