use crate::types::Limits;

#[derive(Debug)]
pub(crate) struct MemType {
    limits: Limits,
}

impl MemType {
    pub(crate) fn new(min_pages: u32, max_pages: u32) -> Self {
        Self {
            limits: Limits {
                min: min_pages,
                max: max_pages,
            },
        }
    }

    pub(crate) fn emit_wat_inline(&self) -> String {
        format!("{} {}", self.limits.min, self.limits.max)
    }
}
