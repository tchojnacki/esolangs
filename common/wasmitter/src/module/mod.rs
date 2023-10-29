mod export;
mod global;
mod import;
mod main;
mod mem;

pub(crate) use self::{
    export::Export,
    global::{Global, GlobalType},
    import::{Import, ImportDesc},
    mem::{Mem, MemType},
};
pub use self::{export::ExportDesc, global::Mutability, main::Module};
