use crate::{func::Func, instructions::Instr, module::Module};

#[derive(Clone, Debug)]
pub struct Expr(pub(crate) Vec<Instr>);

impl From<Vec<Instr>> for Expr {
    fn from(instrs: Vec<Instr>) -> Self {
        Self(instrs)
    }
}

impl From<Instr> for Expr {
    fn from(instr: Instr) -> Self {
        Self(vec![instr])
    }
}

impl Expr {
    pub(crate) fn emit_wat_block(&self, module: &Module, func: &Func, indent: usize) -> String {
        self.0
            .iter()
            .map(|instr| instr.emit_wat_block(module, func, indent))
            .collect()
    }
}
