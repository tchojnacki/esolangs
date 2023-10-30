use std::borrow::Cow;

use crate::text::Id;

pub(crate) trait WasmIndex<'a>: Clone + Copy {
    type Ctx;

    fn resolve(&self, ctx: Self::Ctx) -> u32;
    fn id(&self) -> Id;

    fn id_or_comment(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => format!("(;{};)", self.resolve(ctx)).into(),
        }
    }

    fn id_or_index(&self, ctx: Self::Ctx) -> Cow<'_, str> {
        match self.id().into_option() {
            Some(a) => a.into(),
            None => self.resolve(ctx).to_string().into(),
        }
    }
}
