//! Functionalities related to running Brainfuck code as a part of a Rust program.
//!
//! You should use the generic [`Engine`] type, or the specialized [`StdEngine`] and [`ByteEngine`] types.
//!
//! Available engines:
//! - [`Engine`] - reads inputs from any [`Read`](std::io::Read) type and outputs to any [`Write`](std::io::Write) type
//! - [`StdEngine`] - reads inputs from [`Stdin`](std::io::Stdin) and outputs to [`Stdout`](std::io::Stdout)
//! - [`ByteEngine`] - reads inputs from `&[u8]` and outputs to `&mut Vec<u8>`
//!
//! # Examples
//! ```
//! # use brainlib::{Program, Settings, interpreter::Engine};
//! let source = ",[.,]";
//! let program = Program::compile(source, &Settings::default())?;
//!
//! let input = "ABCD";
//! let mut output = Vec::new();
//! let mut eng = Engine::new_byte_default(program, input.as_bytes(), &mut output);
//! eng.run()?;
//! let output = String::from_utf8_lossy(&output);
//!
//! assert_eq!(input, output);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod engine;
mod runtime_error;

pub use self::{
    engine::{ByteEngine, Engine, StdEngine},
    runtime_error::RuntimeError,
};
