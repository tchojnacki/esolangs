use std::{borrow::Cow, fmt::Debug};

use crate::text::Id;

pub(crate) trait WasmIndex<'a>: Debug + Clone + Copy {
    type Ctx;

    #[must_use]
    fn resolve(&self, ctx: Self::Ctx) -> u32;

    fn id(&self) -> Id;

    #[must_use]
    fn id_or_comment(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => format!("(;{};)", self.resolve(ctx)).into(),
        }
    }

    #[must_use]
    fn id_or_index(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => self.resolve(ctx).to_string().into(),
        }
    }
}
