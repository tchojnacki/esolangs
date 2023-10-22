use crate::{
    indices::{FuncIdx, GlobalIdx, LocalIdx, MemIdx, TypeIdx, WasmIndex},
    instruction::Instr,
    types::{
        Export, ExportDesc, Expr, Func, FuncType, Global, GlobalType, Import, ImportDesc, Limits,
        Mem, MemType, Mutability, ResultType, ValType,
    },
};

#[derive(Default, Debug)]
pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

pub struct FuncScope {
    params: Vec<ValType>,
    results: Vec<ValType>,
    locals: Vec<ValType>,
}

impl FuncScope {
    fn new() -> Self {
        Self {
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
        }
    }
}

impl FuncScope {
    pub fn add_param(&mut self, val_type: ValType) -> LocalIdx {
        self.params.push(val_type);
        LocalIdx::param((self.params.len() - 1) as u32)
    }

    pub fn add_local(&mut self, val_type: ValType) -> LocalIdx {
        self.locals.push(val_type);
        LocalIdx::local((self.locals.len() - 1) as u32)
    }

    pub fn add_result(&mut self, val_type: ValType) {
        self.results.push(val_type);
    }
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn import_count(&self) -> u32 {
        self.imports.len() as u32
    }

    fn resolve_type(
        &mut self,
        params: impl Into<ResultType>,
        results: impl Into<ResultType>,
    ) -> TypeIdx {
        let params = params.into();
        let results = results.into();
        let func_type = FuncType { params, results };
        for (i, ft) in self.types.iter().enumerate() {
            if *ft == func_type {
                return TypeIdx::new(i as u32);
            }
        }
        self.types.push(func_type);
        TypeIdx::new((self.types.len() - 1) as u32)
    }

    pub(crate) fn get_signature(&self, type_idx: TypeIdx) -> &FuncType {
        &self.types[type_idx.resolve(()) as usize]
    }

    pub fn import_func(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        alias: Option<&'static str>,
        params: impl Into<ResultType>,
        results: impl Into<ResultType>,
    ) -> FuncIdx {
        let module = module.into();
        let name = name.into();
        let func_idx = FuncIdx::import(self.imports.len() as u32, alias);
        let desc = ImportDesc::Func {
            type_idx: self.resolve_type(params, results),
            func_idx,
        };
        self.imports.push(Import { module, name, desc });
        func_idx
    }

    pub fn func<B, E>(&mut self, alias: Option<&'static str>, builder: B) -> FuncIdx
    where
        B: FnOnce(&mut FuncScope) -> E,
        E: Into<Expr>,
    {
        let mut scope = FuncScope::new();
        let body = builder(&mut scope).into();
        let type_idx = self.resolve_type(scope.params, scope.results);
        let locals = scope.locals;
        let func_idx = FuncIdx::define(self.funcs.len() as u32, alias);
        self.funcs.push(Func {
            type_idx,
            func_idx,
            locals,
            body,
        });
        func_idx
    }

    pub fn memory(
        &mut self,
        alias: Option<&'static str>,
        min_pages: u32,
        max_pages: u32,
    ) -> MemIdx {
        let mem_idx = MemIdx::new(self.mems.len() as u32, alias);
        self.mems.push(Mem {
            mem_type: MemType {
                limits: Limits {
                    min: min_pages,
                    max: max_pages,
                },
            },
            mem_idx,
        });
        mem_idx
    }

    pub fn global(
        &mut self,
        alias: Option<&'static str>,
        mutability: Mutability,
        val_type: ValType,
        init: Instr,
    ) -> GlobalIdx {
        let global_idx = GlobalIdx::new(self.globals.len() as u32, alias);
        self.globals.push(Global {
            global_type: GlobalType {
                mutability,
                val_type,
            },
            init,
            global_idx,
        });
        global_idx
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
            result.push_str(&mem.emit_wat_block(2));
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