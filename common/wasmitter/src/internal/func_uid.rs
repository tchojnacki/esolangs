use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FuncUid(Uuid);

impl Default for FuncUid {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
