//! Utilities which are not necessary to compile Brainfuck, but might be useful.
//!
//! This includes input/output operations.
//!
//! ```
//! # use std::error::Error;
//! use brainlib::util::{read_byte, write_byte};
//!
//! let input = "X".as_bytes();
//! let mut output = Vec::new();
//!
//! // Copy one byte from input to output
//! let byte = read_byte(input).ok_or("could not read")?;
//! write_byte(&mut output, byte).ok_or("could not write")?;
//! # assert_eq!(output, vec![b'X']);
//! # Ok::<(), Box<dyn Error>>(())
//! ```

mod io;

pub use io::{read_byte, write_byte};
