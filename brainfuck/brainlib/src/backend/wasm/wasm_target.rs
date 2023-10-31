use wasmitter::{
    indices::FuncIdx,
    instruction::{Instr as WI, MemArg},
    module::Module,
    types::I32,
};

use crate::Settings;

/// The target WASM runtime, determining the module structure.
///
/// It determines the origin of the input and output functions,
/// the required memory size, and the header of the `$main` function.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmTarget {
    /// A module that uses imported functions for I/O.
    ///
    /// This target requires you to define the I/O functions yourself, since they
    /// are imported from the produced module. You can, for example, use JavaScript
    /// functions to provide the input and output.
    ///
    /// The following functions are imported:
    /// - `(import "bf" "input" (func (result i32)))`
    /// - `(import "bf" "output" (func (param i32)))`
    ///
    /// They should produce and consume single bytes, respectively. The `i32` type is
    /// used to represent the bytes, since WebAssembly doesn't support `u8` values.
    /// The module uses only the least significant byte of the `i32` values.
    ///
    /// This is the [`WasmTarget::default`] target.
    ///
    /// # Examples
    /// Example output produced by this target:
    /// ```wat
    /// (module
    ///   (import "bf" "input" (func $read_byte (result i32)))
    ///   (import "bf" "output" (func $write_byte (param i32)))
    ///   (memory (;0;) 1 1)
    ///   (global $ptr (mut i32) (i32.const 0))
    ///   (func $main
    ///     ;; example program
    ///   )
    ///   (export "memory" (memory 0))
    ///   (export "_start" (func $main))
    /// )
    Normal,

    /// A module that uses WASI[^1] (The WebAssembly System Interface) for I/O. It can be run using wasmtime[^2].
    ///
    /// This target doesn't require you to define any functions yourself.
    /// Instead, the standard input and output are used through the WASI interface.
    /// The entry point `_start` is called by the runtime, which calls the `$main`
    /// function of [`WasmModule`](crate::wasm::WasmModule). The memory is named `memory`
    /// is also exported, as per the WASI standard.
    ///
    /// The following functions are imported:
    /// - `(import "wasi_unstable" "fd_read" (func (param i32 i32 i32 i32) (result i32)))`[^3]
    /// - `(import "wasi_unstable" "fd_write" (func (param i32 i32 i32 i32) (result i32)))`[^4]
    ///
    /// This target requires more memory and a more complex `$main` function than the [`WasmTarget::Normal`],
    /// since it needs to operate on file descriptors and buffers to conform to the WASI standard.
    ///
    /// # Examples
    /// Example output produced by this target:
    /// ```wat
    /// (module
    ///   (import "wasi_unstable" "fd_read" (func $fd_read (param i32 i32 i32 i32) (result i32)))
    ///   (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
    ///   (memory (;0;) 1 1)
    ///   (global $ptr (mut i32) (i32.const 0))
    ///   (func $read_byte (result i32)
    ///     (i32.const 0)
    ///     (i32.const 30004)
    ///     (i32.const 1)
    ///     (i32.const 30000)
    ///     (call $fd_read)
    ///     (drop)
    ///     (i32.const 30012)
    ///     (i32.load)
    ///   )
    ///   (func $write_byte (param i32)
    ///     (i32.const 30024)
    ///     (local.get 0)
    ///     (i32.store)
    ///     (i32.const 1)
    ///     (i32.const 30016)
    ///     (i32.const 1)
    ///     (i32.const 30000)
    ///     (call $fd_write)
    ///     (drop)
    ///   )
    ///   (func $main
    ///     (i32.const 30004)
    ///     (i32.const 30012)
    ///     (i32.store)
    ///     (i32.const 30008)
    ///     (i32.const 1)
    ///     (i32.store)
    ///     (i32.const 30016)
    ///     (i32.const 30024)
    ///     (i32.store)
    ///     (i32.const 30020)
    ///     (i32.const 1)
    ///     (i32.store)
    ///     ;; example program
    ///   )
    ///   (export "memory" (memory 0))
    ///   (export "_start" (func $main))
    /// )
    /// ```
    ///
    /// [^1]: [WASI](https://wasi.dev)
    ///
    /// [^2]: [wasmtime](https://wasmtime.dev)
    ///
    /// [^3]: [`fd_read`](https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md#fd_read)
    ///
    /// [^4]: [`fd_write`](https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md#fd_write)
    Wasi,
}

impl Default for WasmTarget {
    /// Returns the default target, [`WasmTarget::Normal`].
    fn default() -> Self {
        Self::Normal
    }
}

impl WasmTarget {
    #[must_use]
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
                let read_byte = module.import_func("bf", "input", "$read_byte", (), I32);
                let write_byte = module.import_func("bf", "output", "$write_byte", I32, ());
                (read_byte, write_byte)
            },
            WasmTarget::Wasi => {
                let fd_read = module.import_func(
                    "wasi_unstable",
                    "fd_read",
                    "$fd_read",
                    (I32, I32, I32, I32),
                    I32,
                );

                let fd_write = module.import_func(
                    "wasi_unstable",
                    "fd_write",
                    "$fd_write",
                    (I32, I32, I32, I32),
                    I32,
                );

                let read_byte = module.func("$read_byte", |scope| {
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
                });

                let write_byte = module.func("$write_byte", |scope| {
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
                });

                (read_byte, write_byte)
            },
        }
    }

    #[must_use]
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
