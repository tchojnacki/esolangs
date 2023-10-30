use crate::WasmError;

fn idchar_is_valid(idchar: char) -> bool {
    idchar.is_ascii_alphanumeric() || "!#$%'*+-./:<=>?@\\^_`|~".contains(idchar)
}

#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct Id(Option<&'static str>);

impl Id {
    pub fn none() -> Self {
        Self(None)
    }

    #[must_use]
    pub(crate) fn into_option(self) -> Option<&'static str> {
        self.0
    }

    #[must_use]
    pub(crate) fn validate(&self) -> Option<WasmError> {
        let id = self.0?;
        let mut chars = id.chars().peekable();

        let Some('$') = chars.next() else {
            return Some(WasmError::InvalidIdentifier { id });
        };

        if chars.peek().is_none() {
            return Some(WasmError::InvalidIdentifier { id });
        }

        if chars.any(|c| !idchar_is_valid(c)) {
            return Some(WasmError::InvalidIdentifier { id });
        }

        None
    }
}

impl From<&'static str> for Id {
    fn from(alias: &'static str) -> Self {
        Self(Some(alias))
    }
}
