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
    pub const PAGE_SIZE: u32 = 0x10000;

    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) const fn uid(&self) -> ModuleUid {
        self.uid
    }

    pub(crate) fn func_import_count(&self) -> u32 {
        self.imports.iter().filter(|i| Import::is_func(i)).count() as u32
    }

    pub(crate) fn mem_import_count(&self) -> u32 {
        self.imports.iter().filter(|i| Import::is_mem(i)).count() as u32
    }

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

    pub(crate) fn validate(&self) -> Option<WasmError> {
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

    pub fn import_mem(
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

    pub fn memory(&mut self, id: impl Into<Id>, pages: impl Into<Limits>) -> MemIdx {
        let mem_idx = MemIdx::define(self.uid, self.mems.len() as u32, id.into());
        self.mems.push(Mem::new(pages.into().into(), mem_idx));
        mem_idx
    }

    pub fn global(&mut self, id: impl Into<Id>, mutability: Mut, init: ConstInstr) -> GlobalIdx {
        let global_idx = GlobalIdx::define(self.uid, self.globals.len() as u32, id.into());
        self.globals.push(Global::new(mutability, init, global_idx));
        global_idx
    }

    pub fn export(&mut self, name: impl Into<String>, desc: impl Into<ExportDesc>) {
        self.exports.push(desc.into().into_export(name.into()));
    }

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
