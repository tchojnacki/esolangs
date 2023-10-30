mod export;
mod global;
mod import;
mod main;
mod mem;

pub(crate) use self::{export::Export, global::Global, import::Import, mem::Mem};
pub use self::{export::ExportDesc, main::Module};
