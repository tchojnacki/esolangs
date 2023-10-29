#[derive(PartialEq, Clone, Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl NumType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            NumType::I32 => "i32",
            NumType::I64 => "i64",
            NumType::F32 => "f32",
            NumType::F64 => "f64",
        }
        .into()
    }
}
