use crate::{
    function::Func,
    indices::{FuncIdx, LocalIdx},
    instruction::Expr,
    internal::FuncUid,
    module::Module,
    types::{FuncType, ValType},
};

/// A builder for function's signature.
///
/// This type is obtained through the [`Module::func`] method.
///
/// The scope initially contains no parameters, results, or local variables.
///
/// **NOTE:** You can't currently use [`Id`](crate::text::Id)s for params or locals.
#[must_use]
pub struct FuncScope {
    params: Vec<ValType>,
    results: Vec<ValType>,
    locals: Vec<ValType>,
    func_uid: FuncUid,
}

impl FuncScope {
    pub(crate) fn initialize() -> Self {
        Self {
            params: Vec::new(),
            results: Vec::new(),
            locals: Vec::new(),
            func_uid: FuncUid::default(),
        }
    }

    pub(crate) fn into_func(self, module: &mut Module, func_idx: FuncIdx, body: Expr) -> Func {
        let type_idx = module.resolve_type(FuncType {
            params: self.params.into(),
            results: self.results.into(),
        });

        Func::new(type_idx, func_idx, self.locals, body, self.func_uid)
    }

    /// Adds a parameter of the given [`ValType`] to the function.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, Instr, types::I32};
    /// # let mut module = Module::new();
    /// module.func("$func", |scope| {
    ///     let p = scope.add_param(I32);
    ///     vec![Instr::LocalGet(p), Instr::Drop]
    /// });
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn add_param(&mut self, val_type: ValType) -> LocalIdx {
        self.params.push(val_type);
        LocalIdx::param(self.func_uid, (self.params.len() - 1) as u32)
    }

    /// Adds a local variable of the given [`ValType`] to the function.
    ///
    /// The resulting [`LocalIdx`] can be used to reference the local
    /// variable from instructions in this function's body.
    pub fn add_local(&mut self, val_type: ValType) -> LocalIdx {
        self.locals.push(val_type);
        LocalIdx::local(self.func_uid, (self.locals.len() - 1) as u32)
    }

    /// Adds a result of the given [`ValType`] to the function.
    ///
    /// Note that in WASM, a function can have multiple return values.
    ///
    /// # Examples
    /// ```
    /// # use wasmitter::{Module, Instr, types::I32};
    /// # let mut module = Module::new();
    /// module.func("$func", |scope| {
    ///     scope.add_result(I32);
    ///     Instr::I32Const(42)
    /// });
    /// # assert!(module.validate().is_none());
    /// ```
    pub fn add_result(&mut self, val_type: ValType) {
        self.results.push(val_type);
    }
}
