use crate::types::NumType;

#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    Num(NumType),
}

impl ValType {
    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            ValType::Num(num) => num.emit_wat_inline(),
        }
    }
}

pub const I32: ValType = ValType::Num(NumType::I32);
pub const I64: ValType = ValType::Num(NumType::I64);
pub const F32: ValType = ValType::Num(NumType::F32);
pub const F64: ValType = ValType::Num(NumType::F64);
