//! `brainlib` is a crate used to compile, optimize and interpret [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck).
//!
//! # Examples
//! ```no_run
//! use brainlib::{interpreter::Engine, Program, Settings};
//!
//! let source = "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.";
//! let program = Program::compile(source, &Settings::new())?;
//! Engine::new_std_default(program).run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![warn(missing_docs)]

mod backend;
mod frontend;
pub mod util;

pub use crate::{
    backend::{
        common::{Instruction, Program, Settings},
        interpreter, wasm,
    },
    frontend::ParseError,
};

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    use crate::{
        interpreter::{ByteEngine, RuntimeError, StdEngine},
        Instruction, ParseError, Program, Settings,
    };

    #[test]
    fn check_debuggability() {
        fn assert_debug<T: Debug>() {}

        assert_debug::<Program>();
        assert_debug::<Settings>();
        assert_debug::<ByteEngine>();
        assert_debug::<StdEngine>();
        assert_debug::<Instruction>();
        assert_debug::<ParseError>();
        assert_debug::<RuntimeError>();
    }

    #[test]
    fn check_multithreading_traits() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<Program>();
        assert_send_sync::<Settings>();
        assert_send_sync::<Instruction>();
        assert_send_sync::<ParseError>();
        assert_send_sync::<RuntimeError>();
    }

    #[test]
    fn check_error_goodness() {
        fn assert_error<T: Error + Display + 'static>() {}

        assert_error::<ParseError>();
        assert_error::<RuntimeError>();
    }
}
