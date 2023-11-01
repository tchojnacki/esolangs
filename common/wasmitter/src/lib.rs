//! **wasmitter** (WASM emitter) is a crate for building and emitting WebAssembly[^1] modules programmatically.
//!
//! It can be used on a compiler's backend to emit WASM from the IR.
//!
//! **NOTE:** Currently only a subset of WASM is supported. Namely the following features are *not* supported:
//! - vector types and instructions
//! - reference types and instructions
//! - table types and instructions
//! - external types
//! - custom offsets and alignments for store/load instructions
//! - blocks consuming or producing values
//!
//! # Examples
//! ```no_run
//! # use wasmitter::{instruction::Nn, types::I32, Instr, Module};
//! let mut module = Module::new();
//!
//! let add = module.func("$add", |scope| {
//!     let a = scope.add_param(I32);
//!     let b = scope.add_param(I32);
//!     scope.add_result(I32);
//!
//!     vec![Instr::LocalGet(a), Instr::LocalGet(b), Instr::IAdd(Nn::N32)]
//! });
//!
//! module.export("add", add);
//!
//! println!("{}", module.to_wat()?);
//! # Ok::<(), wasmitter::WasmError>(())
//! ```
//!
//! [^1]: [WebAssembly](https://webassembly.org)

#![warn(missing_docs)]

mod error;
pub mod function;
pub mod indices;
pub mod instruction;
mod internal;
pub mod module;
pub mod text;
pub mod types;

pub use self::{error::WasmError, instruction::Instr, module::Module};
