use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModuleUid(Uuid);

impl Default for ModuleUid {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
