use crate::{
    error::WasmError,
    function::{Func, FuncScope},
    indices::{FuncIdx, GlobalIdx, MemIdx, TypeIdx},
    instruction::{ConstInstr, Expr},
    internal::{ModuleUid, WasmIndex},
    module::{Export, ExportDesc, Global, Import, Mem},
    text::Id,
    types::{FuncType, GlobalType, Limits, Mut, ResultType, ValType},
};

#[derive(Debug, Default)]
pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    imports: Vec<Import>,
    exports: Vec<Export>,
    pub(crate) uid: ModuleUid,
}

impl Module {
    pub const PAGE_SIZE: u32 = 0x10000;

    pub fn new() -> Self {
        Self::default()
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

    pub(crate) fn resolve_type(
        &mut self,
        params: impl Into<ResultType>,
        results: impl Into<ResultType>,
    ) -> TypeIdx {
        let params = params.into();
        let results = results.into();
        let func_type = FuncType { params, results };
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

    pub fn import_func(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        params: impl Into<ResultType>,
        results: impl Into<ResultType>,
    ) -> Result<FuncIdx, WasmError> {
        let id = id.into();
        if let Some(error) = id.validate() {
            return Err(error);
        }

        let func_idx = FuncIdx::import(self.uid, self.func_import_count(), id);
        let type_idx = self.resolve_type(params, results);
        self.imports
            .push(Import::func(module.into(), name.into(), type_idx, func_idx));
        Ok(func_idx)
    }

    pub fn import_mem(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        pages: impl Into<Limits>,
    ) -> Result<MemIdx, WasmError> {
        let id = id.into();
        let pages = pages.into();
        if let Some(error) = id.validate().or(pages.validate()) {
            return Err(error);
        }

        let mem_idx = MemIdx::import(self.uid, self.mem_import_count(), id);
        self.imports.push(Import::mem(
            module.into(),
            name.into(),
            pages.into(),
            mem_idx,
        ));
        Ok(mem_idx)
    }

    pub fn import_global(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        id: impl Into<Id>,
        mutability: Mut,
        val_type: ValType,
    ) -> Result<GlobalIdx, WasmError> {
        let id = id.into();
        if let Some(error) = id.validate() {
            return Err(error);
        }

        let global_idx = GlobalIdx::import(self.uid, self.global_import_count(), id);
        self.imports.push(Import::global(
            module.into(),
            name.into(),
            GlobalType {
                mutability,
                val_type,
            },
            global_idx,
        ));
        Ok(global_idx)
    }

    pub fn func<B, E>(&mut self, id: impl Into<Id>, builder: B) -> Result<FuncIdx, WasmError>
    where
        B: FnOnce(&mut FuncScope) -> E,
        E: Into<Expr>,
    {
        let id = id.into();
        if let Some(error) = id.validate() {
            return Err(error);
        }

        let mut scope = FuncScope::create();
        let body = builder(&mut scope).into();
        let func_idx = FuncIdx::define(self.uid, self.funcs.len() as u32, id);
        let func = scope.into_func(self, func_idx, body);
        if let Some(error) = func.validate(self) {
            return Err(error);
        }

        self.funcs.push(func);
        Ok(func_idx)
    }

    pub fn memory(
        &mut self,
        id: impl Into<Id>,
        pages: impl Into<Limits>,
    ) -> Result<MemIdx, WasmError> {
        let pages = pages.into();
        if let Some(error) = pages.validate() {
            return Err(error);
        }

        let mem_idx = MemIdx::define(self.uid, self.mems.len() as u32, id.into());
        self.mems.push(Mem::new(pages.into(), mem_idx));
        Ok(mem_idx)
    }

    pub fn global(
        &mut self,
        id: impl Into<Id>,
        mutability: Mut,
        init: ConstInstr,
    ) -> Result<GlobalIdx, WasmError> {
        let id = id.into();
        if let Some(error) = id.validate() {
            return Err(error);
        }

        let val_type = init.return_type();
        let global_idx = GlobalIdx::define(self.uid, self.globals.len() as u32, id);
        self.globals.push(Global {
            global_type: GlobalType {
                mutability,
                val_type,
            },
            init,
            global_idx,
        });
        Ok(global_idx)
    }

    pub fn export(&mut self, name: impl Into<String>, desc: impl Into<ExportDesc>) {
        let name = name.into();
        let desc = desc.into();
        self.exports.push(Export { name, desc });
    }

    pub fn to_wat(&self) -> String {
        let mut result = String::new();
        result.push_str("(module\n");

        for import in &self.imports {
            result.push_str(&import.emit_wat_block(self, 2));
        }

        for func in &self.funcs {
            result.push_str(&func.emit_wat_block(self, 2));
        }

        for mem in &self.mems {
            result.push_str(&mem.emit_wat_block(self, 2));
        }

        for global in &self.globals {
            result.push_str(&global.emit_wat_block(self, 2));
        }

        for export in &self.exports {
            result.push_str(&export.emit_wat_block(self, 2));
        }

        result.push_str(")\n");
        result
    }
}
