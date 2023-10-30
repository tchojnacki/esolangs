use crate::types::ValType;

#[derive(PartialEq, Clone, Debug)]
pub struct ResultType(pub(crate) Vec<ValType>);

impl ResultType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        self.0
            .iter()
            .map(|t| t.emit_wat_inline())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl From<ValType> for ResultType {
    fn from(val_type: ValType) -> Self {
        Self(vec![val_type])
    }
}

impl From<Vec<ValType>> for ResultType {
    fn from(val_types: Vec<ValType>) -> Self {
        Self(val_types)
    }
}

impl From<()> for ResultType {
    fn from(_: ()) -> Self {
        Self(Vec::new())
    }
}
