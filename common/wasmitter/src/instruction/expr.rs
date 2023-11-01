use crate::{
    function::Func,
    instruction::{ConstInstr, Instr},
    module::Module,
};

/// A sequence of instructions, with length of zero or more.
///
/// Can be converted to from:
/// - [`Instr`]
/// - [`ConstInstr`]
/// - `Vec<Instr>`
/// - `()`
///
/// Function builder closures return `impl Into<Expr>`,
/// so you can return any of the above types from there.
///
/// # Specification
/// - [Expressions - Structure](https://webassembly.github.io/spec/core/syntax/instructions.html#expressions)
/// - [Expressions - Text Format](https://webassembly.github.io/spec/core/text/instructions.html#expressions)
#[must_use]
#[derive(Debug, Clone)]
pub struct Expr(pub(crate) Vec<Instr>);

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

impl From<Vec<Instr>> for Expr {
    fn from(instrs: Vec<Instr>) -> Self {
        Self(instrs)
    }
}

impl From<()> for Expr {
    fn from(_: ()) -> Self {
        Self(Vec::new())
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
