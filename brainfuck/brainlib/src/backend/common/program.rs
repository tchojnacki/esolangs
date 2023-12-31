use std::fmt::{self, Display};

use crate::{
    backend::common::{emit, optimize, Instruction},
    frontend::{parse, tokenize},
    ParseError, Settings,
};

/// A list of instructions, which is guaranteed to be a valid Brainfuck program.
///
/// The list can be obtained from [`Program::compile`].
#[must_use]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Program(pub(crate) Vec<Instruction>);

impl Program {
    /// Produces a [`Program`] from a string of Brainfuck source code.
    ///
    /// # Errors
    /// Returns [`ParseError`] whenever the source code is invalid.
    ///
    /// # Examples
    /// ```
    /// # use brainlib::{Program, Settings};
    /// let program = Program::compile("+++", &Settings::new().with_debug())?;
    /// assert_eq!(program.to_string(), "+++");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn compile(source: impl AsRef<str>, settings: &Settings) -> Result<Self, ParseError> {
        let tokens = tokenize(source.as_ref());
        let ast = parse(tokens)?;
        let program = emit(&ast);
        Ok(optimize(program, settings))
    }

    /// Same as [`Program::default`], returns an empty [`Program`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the list of [`Instruction`]s contained in the [`Program`].
    ///
    /// # Examples
    /// ```
    /// # use brainlib::{Program, Instruction, Settings};
    /// let program = Program::compile("+++", &Settings::new())?;
    /// assert_eq!(program.code(), &[Instruction::MutCell(3)]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn code(&self) -> &[Instruction] {
        &self.0
    }

    /// Returns the number of [`Instruction`]s contained in the [`Program`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the [`Program`] contains no [`Instruction`]s.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruction in &self.0 {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}
