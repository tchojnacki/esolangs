#[derive(Clone, Copy, Debug)]
pub struct Id(Option<&'static str>);

impl Id {
    pub fn none() -> Self {
        Self(None)
    }

    pub(crate) fn into_option(self) -> Option<&'static str> {
        self.0
    }
}

impl From<&'static str> for Id {
    fn from(alias: &'static str) -> Self {
        Self(Some(alias))
    }
}
