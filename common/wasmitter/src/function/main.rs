use crate::{
    indices::{FuncIdx, TypeIdx},
    instruction::Expr,
    internal::{FuncUid, WasmIndex},
    module::Module,
    types::ValType,
    WasmError,
};

#[derive(Debug)]
pub(crate) struct Func {
    pub(crate) type_idx: TypeIdx,
    pub(crate) func_idx: FuncIdx,
    pub(crate) locals: Vec<ValType>,
    pub(crate) body: Expr,
    pub(crate) uid: FuncUid,
}

impl Func {
    pub(crate) fn validate(&self, module: &Module) -> Option<WasmError> {
        self.func_idx.validate().or(self
            .body
            .0
            .iter()
            .flat_map(|instr| instr.validate(module, self))
            .next())
    }

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
