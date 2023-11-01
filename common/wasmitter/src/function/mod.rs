//! Utilities related to building functions.
//!
//! The internal function type is opaque to the user, functions should instead
//! be built using the [`Module::func`](crate::Module::func) method, which
//! uses the [`FuncScope`] type as a builder. This type can't be directly
//! constructed by the external user.
//!
//! # Examples
//! ```
//! # use wasmitter::{instruction::Nn, types::I32, Instr, Module, function::FuncScope};
//! # let mut module = Module::new();
//! let add = module.func("$add", |scope: &mut FuncScope| {
//!     let a = scope.add_param(I32);
//!     let b = scope.add_param(I32);
//!     scope.add_result(I32);
//!
//!     vec![Instr::LocalGet(a), Instr::LocalGet(b), Instr::IAdd(Nn::N32)]
//! });
//! ```

mod main;
mod scope;

pub(crate) use main::Func;
pub use scope::FuncScope;
