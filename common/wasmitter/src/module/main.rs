use crate::{
    error::WasmError,
    function::{Func, FuncScope},
    indices::{FuncIdx, GlobalIdx, MemIdx, TypeIdx},
    instruction::{ConstInstr, Expr},
    internal::{ModuleUid, WasmIndex},
    module::{Export, ExportDesc, Global, Import, Mem},
    text::Id,
    types::{FuncType, Limits, Mut, ResultType, ValType},
};

/// Represents a WebAssembly module.
///
/// # Examples
/// ```no_run
/// # use wasmitter::{Module, Instr, types::{I32, Mut}, instruction::ConstInstr};
/// let mut module = Module::new();
///
/// let print = module.import_func("console", "log", "$print", I32, ());
///
/// let answer = module.global("$answer", Mut::Const, ConstInstr::I32Const(42));
///
/// let main = module.func("$main", |scope| {
///     vec![Instr::GlobalGet(answer), Instr::Call(print)]
/// });
///
/// module.export("_start", main);
///
/// let wat = module.to_wat()?;
/// println!("{wat}");
/// # Ok::<(), wasmitter::WasmError>(())
/// ```
///
/// # Specification
/// - [Modules - Structure](https://webassembly.github.io/spec/core/syntax/modules.html)
/// - [Modules - Text Format](https://webassembly.github.io/spec/core/text/modules.html)
///
/// # Table of contents
/// - [Functions section](#functions-section)
/// - [Memories section](#memories-section)
/// - [Globals section](#globals-section)
/// - [Imports section](#imports-section)
/// - [Exports section](#exports-section)
#[must_use]
#[derive(Debug, Default)]
pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    imports: Vec<Import>,
    exports: Vec<Export>,
    uid: ModuleUid,
}

impl Module {
    pub(crate) const fn uid(&self) -> ModuleUid {
        self.uid
    }

    #[must_use]
    pub(crate) fn func_import_count(&self) -> u32 {
        self.imports.iter().filter(|i| Import::is_func(i)).count() as u32
    }

    #[must_use]
    pub(crate) fn mem_import_count(&self) -> u32 {
        self.imports.iter().filter(|i| Import::is_mem(i)).count() as u32
    }

    #[must_use]
    pub(crate) fn global_import_count(&self) -> u32 {
        self.imports.iter().filter(|i| Import::is_global(i)).count() as u32
    }

    pub(crate) fn resolve_type(&mut self, func_type: FuncType) -> TypeIdx {
        for (i, ft) in self.types.iter().enumerate() {
            if *ft == func_type {
                return TypeIdx::new(self.uid, i as u32);
            }
        }
        self.types.push(func_type);
        TypeIdx::new(self.uid, (self.types.len() - 1) as u32)
    }

    pub(crate) fn get_signature(&self, type_idx: TypeIdx) -> &FuncType {
        &self.types[type_idx.resolve(self) as usize]
    }
}

impl Module {
    /// Creates a new empty module.
    ///
    /// Same as [`Module::default`].
    ///
    /// Note that two modules created by this function will be different, since each module has a unique identifier.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check for errors in the module.
    ///
    /// Returns the first error found, or `None` if the module is valid.
    ///
    /// Validation is ran automatically when calling [`Module::to_wat`].
    #[must_use]
    pub fn validate(&self) -> Option<WasmError> {
        for func in &self.funcs {
            if let Some(error) = func.validate(self) {
                return Some(error);
            }
        }

        for mem in &self.mems {
            if let Some(error) = mem.validate(self) {
                return Some(error);
            }
        }

        for global in &self.globals {
            if let Some(error) = global.validate(self) {
                return Some(error);
            }
        }

        for import in &self.imports {
            if let Some(error) = import.validate(self) {
                return Some(error);
            }
        }

        None
    }

    /// Emits the module in the WebAssembly text format.
    ///
    /// The sections are emitted in the following order:
    /// 1. Imports
    /// 2. Memories
    /// 3. Globals
    /// 4. Functions
    /// 5. Exports
    ///
    /// # Errors
    /// Returns the first [`WasmError`] found, if any.
    pub fn to_wat(&self) -> Result<String, WasmError> {
        if let Some(error) = self.validate() {
            return Err(error);
        }

        let mut result = String::new();
        result.push_str("(module\n");

        for import in &self.imports {
            result.push_str(&import.emit_wat_block(self, 2));
        }

        for mem in &self.mems {
            result.push_str(&mem.emit_wat_block(self, 2));
        }

        for global in &self.globals {
            result.push_str(&global.emit_wat_block(self, 2));
        }

        for func in &self.funcs {
            result.push_str(&func.emit_wat_block(self, 2));
        }

        for export in &self.exports {
            result.push_str(&export.emit_wat_block(self, 2));
        }

        result.push_str(")\n");
        Ok(result)
    }
}

/// # Functions section
/// - [Functions - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#functions)
/// - [Functions - Text Format](https://webassembly.github.io/spec/core/text/modules.html#functions)
impl Module {
    /// Defines a new function within the module.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, Instr, types::I32, instruction::Nn, indices::FuncIdx};
    /// # let mut module = Module::new();
    /// let sub: FuncIdx = module.func("$subtract", |scope| {
    ///     let a = scope.add_param(I32);
    ///     let b = scope.add_param(I32);
    ///     vec![Instr::LocalGet(a), Instr::LocalGet(b), Instr::ISub(Nn::N32)]
    /// });
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn func<B, E>(&mut self, id: impl Into<Id>, builder: B) -> FuncIdx
    where
        B: FnOnce(&mut FuncScope) -> E,
        E: Into<Expr>,
    {
        let mut scope = FuncScope::initialize();
        let body = builder(&mut scope).into();
        let func_idx = FuncIdx::define(self.uid, self.funcs.len() as u32, id.into());
        let func = scope.into_func(self, func_idx, body);
        self.funcs.push(func);
        func_idx
    }
}

/// # Memories section
/// - [Memories - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#memories)
/// - [Memories - Text Format](https://webassembly.github.io/spec/core/text/modules.html#memories)
impl Module {
    /// WebAssembly page size in bytes - 64 Ki.
    pub const PAGE_SIZE: u32 = 0x10000;

    /// Defines a new memory within the module.
    ///
    /// Note that WASM always implicitly uses the first memory, however more can be defined.
    ///
    /// The `pages` argument takes any type that can be converted into [`Limits`], most commonly
    /// you would use a `u32`, which creates an unbounded memory with initial size of `pages` pages,
    /// or a tuple `(u32, u32)`, which creates a memory with initial size of the first element and
    /// maximum size of the second element.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, types::I32, indices::MemIdx};
    /// # let mut module = Module::new();
    /// let memory: MemIdx = module.memory("$memory", (1, 3));
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn memory(&mut self, id: impl Into<Id>, pages: impl Into<Limits>) -> MemIdx {
        let mem_idx = MemIdx::define(self.uid, self.mems.len() as u32, id.into());
        self.mems.push(Mem::new(pages.into().into(), mem_idx));
        mem_idx
    }
}

/// # Globals section
/// - [Globals - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#globals)
/// - [Globals - Text Format](https://webassembly.github.io/spec/core/text/modules.html#globals)
impl Module {
    /// Defines a new global within the module.
    ///
    /// **NOTE:** Currently only [`ConstInstr`] is supported as the initializer.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, types::Mut, instruction::ConstInstr, indices::GlobalIdx};
    /// # let mut module = Module::new();
    /// let pi: GlobalIdx = module.global("$pi", Mut::Const, ConstInstr::F32Const(3.1415));
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn global(&mut self, id: impl Into<Id>, mutability: Mut, init: ConstInstr) -> GlobalIdx {
        let global_idx = GlobalIdx::define(self.uid, self.globals.len() as u32, id.into());
        self.globals.push(Global::new(mutability, init, global_idx));
        global_idx
    }
}

/// # Imports section
/// - [Imports - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#imports)
/// - [Imports - Text Format](https://webassembly.github.io/spec/core/text/modules.html#imports)
impl Module {
    /// Import a function from another module, returning its [`FuncIdx`].
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, types::I32, indices::FuncIdx};
    /// # let mut module = Module::new();
    /// let fd_write: FuncIdx = module.import_func(
    ///     "wasi_unstable",
    ///     "fd_write",
    ///     "$fd_write",
    ///     (I32, I32, I32, I32),
    ///     I32,
    /// );
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn import_func(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        params: impl Into<ResultType>,
        results: impl Into<ResultType>,
    ) -> FuncIdx {
        let params = params.into();
        let results = results.into();
        let func_idx = FuncIdx::import(self.uid, self.func_import_count(), id.into());
        let type_idx = self.resolve_type(FuncType { params, results });
        self.imports
            .push(Import::func(module.into(), name.into(), type_idx, func_idx));
        func_idx
    }

    /// Import a memory from another module, returning its [`MemIdx`].
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, types::I32, indices::MemIdx};
    /// # let mut module = Module::new();
    /// let memory: MemIdx = module.import_memory("host", "memory", "$memory", 1);
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn import_memory(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        pages: impl Into<Limits>,
    ) -> MemIdx {
        let mem_idx = MemIdx::import(self.uid, self.mem_import_count(), id.into());
        self.imports.push(Import::mem(
            module.into(),
            name.into(),
            pages.into().into(),
            mem_idx,
        ));
        mem_idx
    }

    /// Import a global from another module, returning its [`GlobalIdx`].
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, types::{I32, Mut}, indices::GlobalIdx};
    /// # let mut module = Module::new();
    /// let pi: GlobalIdx = module.import_global("math", "pi", "$pi", Mut::Const, I32);
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn import_global(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        mutability: Mut,
        val_type: ValType,
    ) -> GlobalIdx {
        let global_idx = GlobalIdx::import(self.uid, self.global_import_count(), id.into());
        self.imports.push(Import::global(
            module.into(),
            name.into(),
            mutability,
            val_type,
            global_idx,
        ));
        global_idx
    }
}

/// # Exports section
/// - [Exports - Structure](https://webassembly.github.io/spec/core/syntax/modules.html#exports)
/// - [Exports - Text Format](https://webassembly.github.io/spec/core/text/modules.html#exports)
impl Module {
    /// Export a function, memory or global from the module.
    ///
    /// Any type that can be converted into [`ExportDesc`] can be used as the `desc` argument.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::Module;
    /// # let mut module = Module::new();
    /// let main = module.func("$main", |_| ());
    ///
    /// module.export("_start", main);
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn export(&mut self, name: impl Into<String>, desc: impl Into<ExportDesc>) {
        self.exports.push(desc.into().into_export(name.into()));
    }
}
