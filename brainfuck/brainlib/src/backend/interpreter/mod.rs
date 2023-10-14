//! Functionalities related to running Brainfuck code as a part of a Rust program.
//!
//! You should use the generic [`Engine`] type, or the specialized [`StdEngine`] and [`ByteEngine`] types.
//!
//! Available engines:
//! - [`Engine`] - reads inputs from any [`Read`](std::io::Read) type and outputs to any [`Write`](std::io::Write) type
//! - [`StdEngine`] - reads inputs from [`Stdin`](std::io::Stdin) and outputs to [`Stdout`](std::io::Stdout)
//! - [`ByteEngine`] - reads inputs from `&[u8]` and outputs to `&mut Vec<u8>`

mod engine;
mod runtime_error;

pub use self::{
    engine::{ByteEngine, Engine, StdEngine},
    runtime_error::RuntimeError,
};
