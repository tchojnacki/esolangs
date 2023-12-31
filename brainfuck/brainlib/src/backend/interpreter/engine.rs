use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

use crate::{
    backend::common::{Instruction, Program, Settings},
    interpreter::RuntimeError,
    util::{read_byte, write_byte},
};

/// A generic representation of the interpreter's engine state.
///
/// # Examples
/// ```
/// # use brainlib::{interpreter::Engine, Program, Settings};
/// let settings = Settings::default();
/// let program = Program::compile(",[.,]", &settings)?;
///
/// let engine = Engine::new_std(program, settings);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Engine<In: Read, Out: Write> {
    program: Program,
    pc: usize,
    pointer: u32,
    memory: Box<[u8]>,
    settings: Settings,
    read: In,
    write: Out,
}

/// An engine which operates on [`Stdin`] and [`Stdout`].
pub type StdEngine = Engine<Stdin, Stdout>;

impl StdEngine {
    /// Creates a new [`StdEngine`] with the provided [`Program`] and [`Settings`].
    pub fn new_std(program: Program, settings: Settings) -> Self {
        Self::new(program, settings, stdin(), stdout())
    }

    /// Same as [`StdEngine::new_std`], but uses [`Settings::default`].
    pub fn new_std_default(program: Program) -> Self {
        Self::new_std(program, Settings::default())
    }
}

/// An engine which reads from `&[u8]` and writes to `&mut Vec<u8>`.
pub type ByteEngine<'io> = Engine<&'io [u8], &'io mut Vec<u8>>;

impl<'io> ByteEngine<'io> {
    /// Creates a new [`ByteEngine`] with the provided [`Program`] and [`Settings`].
    pub fn new_byte(
        program: Program,
        settings: Settings,
        input: &'io [u8],
        output: &'io mut Vec<u8>,
    ) -> Self {
        Self::new(program, settings, input, output)
    }

    /// Same as [`ByteEngine::new_byte`], but uses [`Settings::default`].
    pub fn new_byte_default(program: Program, input: &'io [u8], output: &'io mut Vec<u8>) -> Self {
        Self::new_byte(program, Settings::default(), input, output)
    }
}

impl<In: Read, Out: Write> Engine<In, Out> {
    /// Creates a new [`Engine`] with the provided [`Program`], [`Settings`], input and output.
    pub fn new(program: Program, settings: Settings, read: In, write: Out) -> Self {
        Self {
            program,
            pc: 0,
            pointer: 0,
            memory: vec![0; settings.tape_length() as usize].into_boxed_slice(),
            settings,
            read,
            write,
        }
    }

    /// Returns the [`Program`] which the [`Engine`] is executing.
    pub const fn program(&self) -> &Program {
        &self.program
    }

    /// Returns the current program counter.
    #[must_use]
    pub const fn pc(&self) -> usize {
        self.pc
    }

    /// Returns the current cell pointer.
    #[must_use]
    pub const fn pointer(&self) -> u32 {
        self.pointer
    }

    /// Returns a view into the memory (cell vector).
    #[must_use]
    pub const fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Returns the used [`Settings`].
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    #[must_use]
    fn c(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer as usize]
    }

    fn exec(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
        use Instruction as I;
        match instruction {
            I::MutPointer(change) => {
                self.pointer = self.settings.mut_pointer(self.pointer, change).ok_or(
                    RuntimeError::TapeOverflow {
                        from: self.pointer,
                        by: change,
                    },
                )?;
            },
            I::MutCell(change) => {
                let previous = *self.c();
                *self.c() =
                    self.settings
                        .mut_cell(previous, change)
                        .ok_or(RuntimeError::CellOverflow {
                            at: self.pointer,
                            from: *self.c(),
                            by: change,
                        })?;
            },
            I::SetCell(value) => *self.c() = value,
            I::JumpRightZ(offset) =>
                if *self.c() == 0 {
                    self.pc += offset as usize;
                },
            I::JumpLeftNz(offset) =>
                if *self.c() != 0 {
                    self.pc -= offset as usize;
                },
            I::Input => *self.c() = read_byte(&mut self.read).ok_or(RuntimeError::InputError)?,
            I::Output => {
                let value = *self.c();
                write_byte(&mut self.write, value).ok_or(RuntimeError::OutputError)?;
            },
            I::Breakpoint(_) => (), // NOOP
        }
        Ok(())
    }

    /// Executes a single [`Instruction`] and returns it or a [`RuntimeError`].
    #[must_use]
    pub fn step(&mut self) -> Option<Result<Instruction, RuntimeError>> {
        let instruction = *self.program.0.get(self.pc)?;
        self.pc += 1;
        Some(self.exec(instruction).map(|_| instruction))
    }

    /// Runs the [`Engine`] until it halts or a [`RuntimeError`] occurs.
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while let Some(result) = self.step() {
            let _: Instruction = result?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Engine, Instruction as I, Program, RuntimeError, Settings};

    fn assert_interpret(program: Program, input: &str, output: &str) {
        let mut buffer = Vec::new();
        let mut eng = Engine::new_byte_default(program, input.as_bytes(), &mut buffer);
        let res = eng.run();
        assert_eq!(res, Ok(()));
        assert_eq!(buffer, output.as_bytes());
    }

    #[test]
    fn starts_with_zero_at_cell_zero() {
        assert_interpret(Program(vec![I::Output]), "", "\0")
    }

    #[test]
    fn copies_input_with_cat() {
        assert_interpret(
            Program(vec![
                I::Input,
                I::JumpRightZ(3),
                I::Output,
                I::Input,
                I::JumpLeftNz(3),
            ]),
            "Hello, world!",
            "Hello, world!",
        )
    }

    #[test]
    fn reverses_increment_with_decrement() {
        assert_interpret(
            Program(vec![
                I::Input,
                I::MutCell(1),
                I::MutCell(-1),
                I::MutCell(-1),
                I::MutCell(1),
                I::Output,
            ]),
            "x",
            "x",
        )
    }

    #[test]
    fn reverses_right_with_left() {
        assert_interpret(
            Program(vec![
                I::Input,
                I::MutPointer(1),
                I::MutPointer(-1),
                I::MutPointer(-1),
                I::MutPointer(1),
                I::Output,
            ]),
            "A",
            "A",
        )
    }

    #[test]
    fn zeroes_cell_with_loop() {
        assert_interpret(
            Program(vec![
                I::Input,
                I::JumpRightZ(2),
                I::MutCell(-1),
                I::JumpLeftNz(2),
                I::Output,
            ]),
            "X",
            "\0",
        )
    }

    #[test]
    fn wraps_around_mut_pointer_without_strict() {
        let mut eng = Engine::new_std_default(Program(vec![I::MutPointer(-1)]));
        eng.run().unwrap();
        assert_eq!(eng.pointer, Settings::DEFAULT_LENGTH - 1);
    }

    #[test]
    fn reaches_all_values_with_mut_cell() {
        let mut eng = Engine::new_std_default(Program(vec![
            I::MutCell(127),
            I::SetCell(0),
            I::MutCell(-128),
        ]));
        assert_eq!(eng.step(), Some(Ok(I::MutCell(127))));
        assert_eq!(*eng.c(), 127);
        assert_eq!(eng.step(), Some(Ok(I::SetCell(0))));
        assert_eq!(eng.step(), Some(Ok(I::MutCell(-128))));
        assert_eq!(*eng.c(), 128);
    }

    #[test]
    fn returns_error_on_cell_overflow_with_strict() {
        let mut eng = Engine::new_std(Program(vec![I::MutCell(-1)]), Settings::new().with_strict());
        assert_eq!(
            eng.run(),
            Err(RuntimeError::CellOverflow {
                at: 0,
                from: 0,
                by: -1
            })
        );
    }

    #[test]
    fn returns_error_on_pointer_overflow_with_strict() {
        let mut eng = Engine::new_std(
            Program(vec![I::MutPointer(3), I::MutPointer(-5)]),
            Settings::new().with_strict(),
        );
        assert_eq!(
            eng.run(),
            Err(RuntimeError::TapeOverflow { from: 3, by: -5 })
        )
    }

    #[test]
    fn wraps_around_custom_tape_length_without_strict() {
        let mut eng = Engine::new_std(
            Program(vec![I::MutCell(13), I::MutPointer(21)]),
            Settings::try_new(21, false, false).unwrap(),
        );
        eng.run().unwrap();
        assert_eq!(*eng.c(), 13);
    }
}
