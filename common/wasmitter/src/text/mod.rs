//! Utilities for working with the WebAssembly Text Format (WAT)[^1].
//!
//! Info from this module would get ignored in the binary output, but might
//! be useful for debugging, using the text format.
//!
//! [^1]: [Text Format](https://webassembly.github.io/spec/core/text/index.html)

mod id;

pub use id::Id;
