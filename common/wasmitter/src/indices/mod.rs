//! Trivially copyable references to various WASM structures.
//!
//! All of these types implement [`Debug`](std::fmt::Debug), [`Clone`] and [`Copy`].
//!
//! Each index has a context, within which it is valid. [`WasmError::ModuleMismatch`](crate::WasmError::ModuleMismatch) and
//! [`WasmError::FuncMismatch`](crate::WasmError::FuncMismatch) may be returned during validation if the index is used
//! outside of its context.
//!
//! These are all newtypes, which can't be constructed by the user (except for [`LabelIdx`]).
//!
//! # Specification
//! - [Indices - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#indices)
//! - [Indices - Text Format](https://webassembly.github.io/spec/core/text/modules.html#indices)

mod func_idx;
mod global_idx;
mod label_idx;
mod local_idx;
mod mem_idx;
mod type_idx;

pub(crate) use self::type_idx::TypeIdx;
pub use self::{
    func_idx::FuncIdx, global_idx::GlobalIdx, label_idx::LabelIdx, local_idx::LocalIdx,
    mem_idx::MemIdx,
};
