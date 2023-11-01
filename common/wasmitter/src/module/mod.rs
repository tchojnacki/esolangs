//! Types used to represent a WebAssembly module.
//!
//! The [`Module`] type is also re-exported from the root of the crate.
//!
//! **NOTE:** Currently, only a subset of the sections is supported.
//!
//! # Specification
//! - [Modules - Structure](https://webassembly.github.io/spec/core/syntax/modules.html)
//! - [Modules - Text Format](https://webassembly.github.io/spec/core/text/modules.html)

mod export;
mod global;
mod import;
mod main;
mod mem;

pub(crate) use self::{export::Export, global::Global, import::Import, mem::Mem};
pub use self::{export::ExportDesc, main::Module};
