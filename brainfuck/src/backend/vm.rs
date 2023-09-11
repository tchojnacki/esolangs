use crate::{
    backend::{
        instruction::{Instruction, Program},
        settings::Settings,
    },
    util::{read_byte, write_byte},
};
use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

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

impl VirtualMachine<Stdin, Stdout> {
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

    #[must_use]
    fn c(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer]
    }

    fn exec(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
        use Instruction as I;
        match instruction {
            I::MutPointer(offset) => {
                let new = self.pointer as i32 + offset;
                if self.settings.strict() && (new < 0 || new >= self.memory.len() as i32) {
                    return Err(RuntimeError::TapeOverflow {
                        from: self.pointer,
                        by: offset,
                    });
                }
                self.pointer = new.rem_euclid(self.memory.len() as i32) as usize;
            }
            I::MutCell(offset) => {
                if self.settings.strict() {
                    *self.c() =
                        self.c()
                            .checked_add_signed(offset)
                            .ok_or(RuntimeError::CellOverflow {
                                at: self.pointer,
                                from: *self.c(),
                                by: offset,
                            })?;
                } else {
                    *self.c() = self.c().wrapping_add_signed(offset);
                }
            }
            I::SetCell(value) => {
                *self.c() = value;
            }
            I::RelJumpRightZero(offset) => {
                if *self.c() == 0 {
                    self.pc += offset as usize;
                }
            }
            I::RelJumpLeftNotZero(offset) => {
                if *self.c() != 0 {
                    self.pc -= offset as usize;
                }
            }
            I::Input => {
                *self.c() = read_byte(&mut self.read).ok_or(RuntimeError::InputError)?;
            }
            I::Output => {
                let value = *self.c();
                write_byte(&mut self.write, value).ok_or(RuntimeError::OutputError)?;
            }
        }
        Ok(())
    }

    #[must_use]
    fn step(&mut self) -> Option<Result<(), RuntimeError>> {
        let instruction = *self.program.get(self.pc)?;
        self.pc += 1;
        Some(self.exec(instruction))
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while let Some(result) = self.step() {
            result?;
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
    fn cat_copies_input() {
        assert_interpret(
            vec![
                I::Input,
                I::RelJumpRightZero(3),
                I::Output,
                I::Input,
                I::RelJumpLeftNotZero(3),
            ],
            "Hello, world!",
            "Hello, world!",
        )
    }

    #[test]
    fn decrement_reverses_increment() {
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
    fn left_reverses_right() {
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
    fn loop_zeroes_cell() {
        assert_interpret(
            vec![
                I::Input,
                I::RelJumpRightZero(2),
                I::MutCell(-1),
                I::RelJumpLeftNotZero(2),
                I::Output,
            ],
            "X",
            "\0",
        )
    }

    #[test]
    fn mut_pointer_wraps_around() {
        let mut vm = VirtualMachine::new_std_default(vec![I::MutPointer(-1)]);
        vm.run().unwrap();
        assert_eq!(vm.pointer as u32, Settings::DEFAULT_LENGTH - 1);
    }

    #[test]
    fn mut_cell_reaches_all_values() {
        let mut vm =
            VirtualMachine::new_std_default(vec![I::MutCell(127), I::SetCell(0), I::MutCell(-128)]);
        vm.step().unwrap().unwrap();
        assert_eq!(*vm.c(), 127);
        vm.step().unwrap().unwrap();
        vm.step().unwrap().unwrap();
        assert_eq!(*vm.c(), 128);
    }

    #[test]
    fn cell_overflows_return_error_in_strict_mode() {
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
    fn pointer_overflows_return_error_in_strict_mode() {
        let mut vm = VirtualMachine::new_std_strict(vec![I::MutPointer(3), I::MutPointer(-5)]);
        assert_eq!(
            vm.run(),
            Err(RuntimeError::TapeOverflow { from: 3, by: -5 })
        )
    }

    #[test]
    fn wrap_around_works_on_custom_length() {
        let mut vm = VirtualMachine::new_std(
            vec![I::MutCell(13), I::MutPointer(21)],
            Settings::try_new(21, false).unwrap(),
        );
        vm.run().unwrap();
        assert_eq!(*vm.c(), 13);
    }
}
