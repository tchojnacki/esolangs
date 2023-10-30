use crate::{
    function::Func,
    indices::{FuncIdx, LocalIdx},
    instruction::Expr,
    internal::FuncUid,
    module::Module,
    types::ValType,
};

pub struct FuncScope {
    params: Vec<ValType>,
    results: Vec<ValType>,
    locals: Vec<ValType>,
    func_uid: FuncUid,
}

impl FuncScope {
    pub(crate) fn create() -> Self {
        Self {
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
            func_uid: FuncUid::default(),
        }
    }

    pub(crate) fn into_func(self, module: &mut Module, func_idx: FuncIdx, body: Expr) -> Func {
        Func {
            type_idx: module.resolve_type(self.params, self.results),
            func_idx,
            locals: self.locals,
            body,
            uid: self.func_uid,
        }
    }

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
