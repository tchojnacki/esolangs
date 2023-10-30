use crate::{
    function::Func,
    instruction::{ConstInstr, Instr},
    module::Module,
};

#[must_use]
#[derive(Debug, Clone)]
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

impl From<ConstInstr> for Expr {
    fn from(instr: ConstInstr) -> Self {
        Self(vec![instr.into()])
    }
}

impl Expr {
    #[must_use]
    pub(crate) fn emit_wat_block(&self, module: &Module, func: &Func, indent: usize) -> String {
        self.0
            .iter()
            .map(|instr| instr.emit_wat_block(module, func, indent))
            .collect()
    }
}
