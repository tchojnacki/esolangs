#[must_use]
#[derive(Debug, Clone, Copy)]
pub(crate) enum IndexKind {
    Imported(u32),
    Defined(u32),
}

impl IndexKind {
    #[must_use]
    pub(crate) fn resolve(&self, import_count: u32) -> u32 {
        match *self {
            IndexKind::Imported(idx) => idx,
            IndexKind::Defined(idx) => import_count + idx,
        }
    }
}
