use wasmitter::{
    indices::FuncIdx,
    instruction::{Instr as WI, MemArg},
    module::Module,
    types::I32,
};

use crate::Settings;

#[derive(Clone, Copy, Debug)]
pub enum WasmTarget {
    Normal,
    Wasi,
}

impl Default for WasmTarget {
    fn default() -> Self {
        Self::Normal
    }
}

impl WasmTarget {
    pub(crate) fn required_pages(&self, settings: &Settings) -> u32 {
        let required_bytes = settings.tape_length()
            + match self {
                WasmTarget::Normal => 0,
                WasmTarget::Wasi => 28,
            };

        (required_bytes + Module::PAGE_SIZE - 1) / Module::PAGE_SIZE
    }

    pub(crate) fn inject_io_funcs(
        &self,
        module: &mut Module,
        settings: &Settings,
    ) -> (FuncIdx, FuncIdx) {
        match self {
            WasmTarget::Normal => {
                let read_byte = module
                    .import_func("bf", "input", "$read_byte", (), I32)
                    .expect("invalid identifier");
                let write_byte = module
                    .import_func("bf", "output", "$write_byte", I32, ())
                    .expect("invalid identifier");
                (read_byte, write_byte)
            },
            WasmTarget::Wasi => {
                let fd_read = module
                    .import_func(
                        "wasi_unstable",
                        "fd_read",
                        "$fd_read",
                        vec![I32, I32, I32, I32],
                        I32,
                    )
                    .expect("invalid identifier");

                let fd_write = module
                    .import_func(
                        "wasi_unstable",
                        "fd_write",
                        "$fd_write",
                        vec![I32, I32, I32, I32],
                        I32,
                    )
                    .expect("invalid identifier");

                let read_byte = module
                    .func("$read_byte", |scope| {
                        scope.add_result(I32);
                        vec![
                            WI::I32Const(0),
                            WI::I32Const(settings.tape_length() + 4),
                            WI::I32Const(1),
                            WI::I32Const(settings.tape_length()),
                            WI::Call(fd_read),
                            WI::Drop,
                            WI::I32Const(settings.tape_length() + 12),
                            WI::I32Load(MemArg::default()),
                        ]
                    })
                    .expect("failed to create $read_byte");

                let write_byte = module
                    .func("$write_byte", |scope| {
                        let value = scope.add_param(I32);
                        vec![
                            WI::I32Const(settings.tape_length() + 24),
                            WI::LocalGet(value),
                            WI::I32Store(MemArg::default()),
                            WI::I32Const(1),
                            WI::I32Const(settings.tape_length() + 16),
                            WI::I32Const(1),
                            WI::I32Const(settings.tape_length()),
                            WI::Call(fd_write),
                            WI::Drop,
                        ]
                    })
                    .expect("failed to create $write_byte");

                (read_byte, write_byte)
            },
        }
    }

    pub(crate) fn main_header(&self, settings: &Settings) -> Vec<WI> {
        match self {
            WasmTarget::Normal => Vec::new(),
            WasmTarget::Wasi => vec![
                WI::I32Const(settings.tape_length() + 4),
                WI::I32Const(settings.tape_length() + 12),
                WI::I32Store(MemArg::default()),
                WI::I32Const(settings.tape_length() + 8),
                WI::I32Const(1),
                WI::I32Store(MemArg::default()),
                WI::I32Const(settings.tape_length() + 16),
                WI::I32Const(settings.tape_length() + 24),
                WI::I32Store(MemArg::default()),
                WI::I32Const(settings.tape_length() + 20),
                WI::I32Const(1),
                WI::I32Store(MemArg::default()),
            ],
        }
    }
}
