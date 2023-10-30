use crate::{
    indices::{FuncIdx, TypeIdx},
    instruction::Expr,
    internal::{FuncUid, WasmIndex},
    module::Module,
    types::ValType,
    WasmError,
};

#[must_use]
#[derive(Debug)]
pub(crate) struct Func {
    type_idx: TypeIdx,
    func_idx: FuncIdx,
    locals: Vec<ValType>,
    body: Expr,
    uid: FuncUid,
}

impl Func {
    pub(crate) fn new(
        type_idx: TypeIdx,
        func_idx: FuncIdx,
        locals: Vec<ValType>,
        body: Expr,
        uid: FuncUid,
    ) -> Self {
        Self {
            type_idx,
            func_idx,
            locals,
            body,
            uid,
        }
    }

    pub(crate) const fn type_idx(&self) -> TypeIdx {
        self.type_idx
    }

    pub(crate) const fn uid(&self) -> FuncUid {
        self.uid
    }

    #[must_use]
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.type_idx
            .validate(module)
            .or(self.func_idx.validate(module))
            .or(self
                .body
                .0
                .iter()
                .flat_map(|instr| instr.validate(module, self, 0))
                .next())
    }

    #[must_use]
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        let tab = " ".repeat(indent);
        let mut result = String::new();
        let func_type = module.get_signature(self.type_idx);

        result.push_str(&format!(
            "{tab}(func {} {}\n",
            self.func_idx.id_or_comment(module),
            func_type.emit_wat_inline()
        ));

        for local in &self.locals {
            result.push_str(&format!("{tab}  (local {})\n", local.emit_wat_inline()));
        }

        result.push_str(&self.body.emit_wat_block(module, self, indent + 2));

        result.push_str(&format!("{tab})\n"));

        result
    }
}
