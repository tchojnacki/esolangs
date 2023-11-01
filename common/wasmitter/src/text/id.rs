use crate::WasmError;

fn idchar_is_valid(idchar: char) -> bool {
    idchar.is_ascii_alphanumeric() || "!#$%'*+-./:<=>?@\\^_`|~".contains(idchar)
}

/// Optional identifier that stand in lieu of an index.
///
/// This provides a textual alias for a [`FuncIdx`](crate::indices::FuncIdx),
/// [`MemIdx`](crate::indices::MemIdx) or [`GlobalIdx`](crate::indices::GlobalIdx).
///
/// The identifier is emitted to the text format, but is ignored elsewhere.
///
/// # Examples
/// ```
/// # use wasmitter::text::Id;
/// let id1 = Id::from("$my_func");
/// let id2 = Id::none();
/// ```
#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct Id(Option<&'static str>);

impl Id {
    /// Creates an non-existent identifier. This will use the index notation instead.
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
