use crate::types::ValType;

#[must_use]
#[derive(Debug)]
pub enum Mut {
    Const,
    Var,
}

#[must_use]
#[derive(Debug)]
pub(crate) struct GlobalType {
    pub(crate) mutability: Mut,
    pub(crate) val_type: ValType,
}

impl GlobalType {
    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self.mutability {
            Mut::Const => self.val_type.emit_wat_inline(),
            Mut::Var => format!("(mut {})", self.val_type.emit_wat_inline()),
        }
    }
}
