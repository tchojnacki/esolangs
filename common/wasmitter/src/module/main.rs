use super::global;
use crate::{
    error::WasmError,
    func::Func,
    indices::{FuncIdx, GlobalIdx, LocalIdx, MemIdx, TypeIdx, WasmIndex},
    instructions::ConstInstr,
    internal::{FuncUid, ModuleUid},
    module::{
        Export, ExportDesc, Global, GlobalType, Import, ImportDesc, Mem, MemType, Mutability,
    },
    text::Id,
    types::{Expr, FuncType, Limits, ResultType, ValType, F32, F64, I32, I64},
};

#[derive(Default)]
pub struct FuncScope {
    params: Vec<ValType>,
    results: Vec<ValType>,
    locals: Vec<ValType>,
    func_uid: FuncUid,
}

impl FuncScope {
    pub fn add_param(&mut self, val_type: ValType) -> LocalIdx {
        self.params.push(val_type);
        LocalIdx::param(self.func_uid, (self.params.len() - 1) as u32)
    }

    pub fn add_local(&mut self, val_type: ValType) -> LocalIdx {
        self.locals.push(val_type);
        LocalIdx::local(self.func_uid, (self.locals.len() - 1) as u32)
    }

    pub fn add_result(&mut self, val_type: ValType) {
        self.results.push(val_type);
    }
}

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
    ) -> FuncIdx {
        let module = module.into();
        let name = name.into();
        let func_idx = FuncIdx::import(self.uid, self.imports.len() as u32, id.into());
        let desc = ImportDesc::Func {
            type_idx: self.resolve_type(params, results),
            func_idx,
        };
        self.imports.push(Import { module, name, desc });
        func_idx
    }

    pub fn func<B, E>(&mut self, id: impl Into<Id>, builder: B) -> Result<FuncIdx, WasmError>
    where
        B: FnOnce(&mut FuncScope) -> E,
        E: Into<Expr>,
    {
        let mut scope = FuncScope::default();
        let body = builder(&mut scope).into();
        let type_idx = self.resolve_type(scope.params, scope.results);
        let locals = scope.locals;
        let func_idx = FuncIdx::define(self.uid, self.funcs.len() as u32, id.into());
        let uid = scope.func_uid;
        let func = Func {
            type_idx,
            func_idx,
            locals,
            body,
            uid,
        };

        if let Some(error) = func
            .body
            .0
            .iter()
            .flat_map(|instr| instr.validate(self, &func))
            .next()
        {
            return Err(error);
        }

        self.funcs.push(func);
        Ok(func_idx)
    }

    pub fn memory(&mut self, id: impl Into<Id>, min_pages: u32, max_pages: u32) -> MemIdx {
        let mem_idx = MemIdx::new(self.uid, self.mems.len() as u32, id.into());
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
        id: impl Into<Id>,
        mutability: Mutability,
        init: ConstInstr,
    ) -> GlobalIdx {
        let val_type = init.return_type();
        let global_idx = GlobalIdx::new(self.uid, self.globals.len() as u32, id.into());
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
