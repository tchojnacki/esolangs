//! Instructions and related types.
//!
//! The main types are:
//! - [`Instr`] - used in functions
//! - [`ConstInstr`] - used in globals
//!
//! The other types are used as arguments to those instructions.
//!
//! The [`Instr`] type is also re-exported from the root of the crate.
//!
//! # Examples
//! ```
//! # use wasmitter::{Module, types::{I32, Mut}, instruction::{BlockType, ConstInstr, Instr, Nn}};
//! # let mut module = Module::new();
//! let my_global = module.global("$my_global", Mut::Var, ConstInstr::I32Const(42));
//!
//! module.func("$func", |scope| {
//!     vec![Instr::Block(
//!         BlockType::default(),
//!         vec![
//!             Instr::GlobalGet(my_global),
//!             Instr::I32Const(5),
//!             Instr::IAdd(Nn::N32),
//!             Instr::BrIf(0.into()),
//!         ],
//!     )]
//! });
//! # assert!(module.validate().is_none());
//! ```
//!
//! # Specification
//! - [Instructions - Structure](https://webassembly.github.io/spec/core/syntax/instructions.html)
//! - [Instructions - Text Format](https://webassembly.github.io/spec/core/text/instructions.html)

mod block_type;
mod const_instr;
mod expr;
mod instr;
mod mem_arg;
mod nn;
mod sx;

pub use self::{
    block_type::BlockType, const_instr::ConstInstr, expr::Expr, instr::Instr, mem_arg::MemArg,
    nn::Nn, sx::Sx,
};
