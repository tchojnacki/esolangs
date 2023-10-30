use crate::types::ResultType;

#[derive(PartialEq, Clone, Debug)]
pub struct FuncType {
    pub params: ResultType,
    pub results: ResultType,
}

impl FuncType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        let construct_part = |result_type: &ResultType, name: &str| {
            let wat = result_type.emit_wat_inline();
            if wat.is_empty() {
                String::new()
            } else {
                format!("({name} {wat})")
            }
        };

        [
            construct_part(&self.params, "param"),
            construct_part(&self.results, "result"),
        ]
        .iter()
        .cloned()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
    }
}