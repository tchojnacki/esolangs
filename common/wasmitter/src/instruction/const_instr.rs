use crate::{
    instruction::Instr,
    types::{ValType, F32, F64, I32, I64},
};

/// An instruction which can be used in global initialization.
///
/// **NOTE:** Currently, only a subset of those instructions is supported and constant instructions
/// can't be composed into constant expressions. Namely, you can only initialize globals to numeric values.
///
/// # Examples
/// ```
/// # use wasmitter::{Module, instruction::ConstInstr, types::Mut};
/// # let mut module = Module::new();
/// module.global("$my_global", Mut::Const, ConstInstr::I32Const(42));
/// # assert!(module.validate().is_none());
/// ```
///
/// # Specification
/// - [Constant Expressions - Validation](https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions)
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ConstInstr {
    /// `i32.const u32`
    I32Const(u32),
    /// `i64.const u64`
    I64Const(u64),
    /// `f32.const f32`
    F32Const(f32),
    /// `f64.const f64`
    F64Const(f64),
}

impl ConstInstr {
    pub(crate) const fn return_type(&self) -> ValType {
        match self {
            Self::I32Const(_) => I32,
            Self::I64Const(_) => I64,
            Self::F32Const(_) => F32,
            Self::F64Const(_) => F64,
        }
    }

    #[must_use]
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            Self::I32Const(val) => format!("(i32.const {})", *val as i32),
            Self::I64Const(val) => format!("(i64.const {})", *val as i64),
            Self::F32Const(val) => format!("(f32.const {val})"),
            Self::F64Const(val) => format!("(f64.const {val})"),
        }
    }
}

impl From<ConstInstr> for Instr {
    fn from(value: ConstInstr) -> Instr {
        match value {
            ConstInstr::I32Const(val) => Instr::I32Const(val),
            ConstInstr::I64Const(val) => Instr::I64Const(val),
            ConstInstr::F32Const(val) => Instr::F32Const(val),
            ConstInstr::F64Const(val) => Instr::F64Const(val),
        }
    }
}
