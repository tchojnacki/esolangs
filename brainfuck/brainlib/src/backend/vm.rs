use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

use crate::{
    backend::{
        instruction::{Instruction, Program},
        settings::Settings,
    },
    util::{read_byte, write_byte},
};

#[must_use]
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InputError,
    OutputError,
    TapeOverflow { from: usize, by: i32 },
    CellOverflow { at: usize, from: u8, by: i8 },
}

#[must_use]
pub struct VirtualMachine<R: Read, W: Write> {
    program: Program,
    pc: usize,
    pointer: usize,
    memory: Box<[u8]>,
    settings: Settings,
    read: R,
    write: W,
}

pub type VirtualMachineStd = VirtualMachine<Stdin, Stdout>;

impl VirtualMachineStd {
    pub fn new_std(program: Program, settings: Settings) -> Self {
        Self::new(program, settings, stdin(), stdout())
    }

    pub fn new_std_default(program: Program) -> Self {
        Self::new_std(program, Settings::default())
    }

    pub fn new_std_strict(program: Program) -> Self {
        Self::new_std(program, Settings::default_strict())
    }
}

impl<R: Read, W: Write> VirtualMachine<R, W> {
    pub fn new(program: Program, settings: Settings, read: R, write: W) -> Self {
        Self {
            program,
            pc: 0,
            pointer: 0,
            memory: vec![0; settings.tape_length()].into_boxed_slice(),
            settings,
            read,
            write,
        }
    }

    pub const fn program(&self) -> &Program {
        &self.program
    }

    pub const fn pc(&self) -> usize {
        self.pc
    }

    pub const fn pointer(&self) -> usize {
        self.pointer
    }

    pub const fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    #[must_use]
    fn c(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer]
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

    #[must_use]
    pub fn step(&mut self) -> Option<Result<Instruction, RuntimeError>> {
        let instruction = *self.program.get(self.pc)?;
        self.pc += 1;
        Some(self.exec(instruction).map(|_| instruction))
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while let Some(result) = self.step() {
            let _: Instruction = result?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction as I, Program, RuntimeError, Settings, VirtualMachine};

    fn assert_interpret(program: Program, input: &str, output: &str) {
        let mut buffer = Vec::new();
        let mut vm =
            VirtualMachine::new(program, Settings::default(), input.as_bytes(), &mut buffer);
        let res = vm.run();
        assert_eq!(res, Ok(()));
        assert_eq!(buffer, output.as_bytes());
    }

    #[test]
    fn starts_with_zero_at_cell_zero() {
        assert_interpret(vec![I::Output], "", "\0")
    }

    #[test]
    fn copies_input_with_cat() {
        assert_interpret(
            vec![
                I::Input,
                I::JumpRightZ(3),
                I::Output,
                I::Input,
                I::JumpLeftNz(3),
            ],
            "Hello, world!",
            "Hello, world!",
        )
    }

    #[test]
    fn reverses_increment_with_decrement() {
        assert_interpret(
            vec![
                I::Input,
                I::MutCell(1),
                I::MutCell(-1),
                I::MutCell(-1),
                I::MutCell(1),
                I::Output,
            ],
            "x",
            "x",
        )
    }

    #[test]
    fn reverses_right_with_left() {
        assert_interpret(
            vec![
                I::Input,
                I::MutPointer(1),
                I::MutPointer(-1),
                I::MutPointer(-1),
                I::MutPointer(1),
                I::Output,
            ],
            "A",
            "A",
        )
    }

    #[test]
    fn zeroes_cell_with_loop() {
        assert_interpret(
            vec![
                I::Input,
                I::JumpRightZ(2),
                I::MutCell(-1),
                I::JumpLeftNz(2),
                I::Output,
            ],
            "X",
            "\0",
        )
    }

    #[test]
    fn wraps_around_mut_pointer_without_strict() {
        let mut vm = VirtualMachine::new_std_default(vec![I::MutPointer(-1)]);
        vm.run().unwrap();
        assert_eq!(vm.pointer as u32, Settings::DEFAULT_LENGTH - 1);
    }

    #[test]
    fn reaches_all_values_with_mut_cell() {
        let mut vm =
            VirtualMachine::new_std_default(vec![I::MutCell(127), I::SetCell(0), I::MutCell(-128)]);
        assert_eq!(vm.step(), Some(Ok(I::MutCell(127))));
        assert_eq!(*vm.c(), 127);
        assert_eq!(vm.step(), Some(Ok(I::SetCell(0))));
        assert_eq!(vm.step(), Some(Ok(I::MutCell(-128))));
        assert_eq!(*vm.c(), 128);
    }

    #[test]
    fn returns_error_on_cell_overflow_with_strict() {
        let mut vm = VirtualMachine::new_std_strict(vec![I::MutCell(-1)]);
        assert_eq!(
            vm.run(),
            Err(RuntimeError::CellOverflow {
                at: 0,
                from: 0,
                by: -1
            })
        );
    }

    #[test]
    fn returns_error_on_pointer_overflow_with_strict() {
        let mut vm = VirtualMachine::new_std_strict(vec![I::MutPointer(3), I::MutPointer(-5)]);
        assert_eq!(
            vm.run(),
            Err(RuntimeError::TapeOverflow { from: 3, by: -5 })
        )
    }

    #[test]
    fn wraps_around_custom_tape_length_without_strict() {
        let mut vm = VirtualMachine::new_std(
            vec![I::MutCell(13), I::MutPointer(21)],
            Settings::try_new(21, false, false).unwrap(),
        );
        vm.run().unwrap();
        assert_eq!(*vm.c(), 13);
    }
}