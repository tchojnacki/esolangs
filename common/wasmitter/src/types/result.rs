use crate::types::ValType;

#[must_use]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ResultType(Vec<ValType>);

impl ResultType {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
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

impl From<(ValType,)> for ResultType {
    fn from((v0,): (ValType,)) -> Self {
        Self(vec![v0])
    }
}

impl From<(ValType, ValType)> for ResultType {
    fn from((v0, v1): (ValType, ValType)) -> Self {
        Self(vec![v0, v1])
    }
}

impl From<(ValType, ValType, ValType)> for ResultType {
    fn from((v0, v1, v2): (ValType, ValType, ValType)) -> Self {
        Self(vec![v0, v1, v2])
    }
}

impl From<(ValType, ValType, ValType, ValType)> for ResultType {
    fn from((v0, v1, v2, v3): (ValType, ValType, ValType, ValType)) -> Self {
        Self(vec![v0, v1, v2, v3])
    }
}

impl From<(ValType, ValType, ValType, ValType, ValType)> for ResultType {
    fn from((v0, v1, v2, v3, v4): (ValType, ValType, ValType, ValType, ValType)) -> Self {
        Self(vec![v0, v1, v2, v3, v4])
    }
}

impl From<(ValType, ValType, ValType, ValType, ValType, ValType)> for ResultType {
    fn from(
        (v0, v1, v2, v3, v4, v5): (ValType, ValType, ValType, ValType, ValType, ValType),
    ) -> Self {
        Self(vec![v0, v1, v2, v3, v4, v5])
    }
}

impl
    From<(
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
    )> for ResultType
{
    fn from(
        (v0, v1, v2, v3, v4, v5, v6): (
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
        ),
    ) -> Self {
        Self(vec![v0, v1, v2, v3, v4, v5, v6])
    }
}

impl
    From<(
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
        ValType,
    )> for ResultType
{
    fn from(
        (v0, v1, v2, v3, v4, v5, v6, v7): (
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
            ValType,
        ),
    ) -> Self {
        Self(vec![v0, v1, v2, v3, v4, v5, v6, v7])
    }
}
