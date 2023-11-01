//! Functionalities related to transpilation from Brainfuck to WASM.
//!
//! > WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine.
//! > Wasm is designed as a portable compilation target for programming languages, enabling deployment
//! > on the web for client and server applications. [^1]
//!
//! The features are exposed through the [`WasmModule`] and [`WasmTarget`] types.
//!
//! **NOTE:** Currently only WAT[^2] (WebAssembly Text) format is supported.
//!
//! # Examples
//! ```
//! # use std::io::stdout;
//! use brainlib::{
//!     wasm::{WasmModule, WasmTarget},
//!     Program, Settings,
//! };
//!
//! let settings = Settings::default();
//! let source = ",[.,]";
//! let program = Program::compile(source, &settings)?;
//!
//! let module = WasmModule::compile_from(&program, WasmTarget::Normal, &settings);
//! module.emit_wat(stdout())?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [^1]: [WebAssembly](https://webassembly.org)
//!
//! [^2]: [Understanding the text format - MDN](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)

mod wasm_module;
mod wasm_target;

pub use self::{wasm_module::WasmModule, wasm_target::WasmTarget};
