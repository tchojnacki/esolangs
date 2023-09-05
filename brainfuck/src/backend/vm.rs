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
    pub fn new_std(program: Program) -> Self {
        Self::new(program, Settings::default(), stdin(), stdout())
    }
}

impl<R: Read, W: Write> VirtualMachine<R, W> {
    pub fn new(program: Program, settings: Settings, read: R, write: W) -> Self {
        Self {
            program,
            pc: 0,
            pointer: 0,
            memory: vec![0; settings.tape_length as usize].into_boxed_slice(),
            settings,
            read,
            write,
        }
    }

    #[must_use]
    fn cell(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer]
    }

    #[must_use]
    fn step(&mut self) -> Option<Result<(), RuntimeError>> {
        let instruction = *self.program.get(self.pc)?;
        self.pc += 1;

        use Instruction as I;
        match instruction {
            I::MutPointer(offset) => {
                self.pointer =
                    (self.pointer as i32 + offset).rem_euclid(self.memory.len() as i32) as usize;
            }
            I::MutCell(offset) => {
                *self.cell() = self.cell().wrapping_add_signed(offset);
            }
            I::SetCell(value) => {
                *self.cell() = value;
            }
            I::RelJumpRightZero(offset) => {
                if *self.cell() == 0 {
                    self.pc += offset as usize;
                }
            }
            I::RelJumpLeftNotZero(offset) => {
                if *self.cell() != 0 {
                    self.pc -= offset as usize;
                }
            }
            I::Input => {
                match read_byte(&mut self.read) {
                    Some(value) => *self.cell() = value,
                    None => return Some(Err(RuntimeError::InputError)),
                };
            }
            I::Output => {
                let value = *self.cell();
                let Some(()) = write_byte(&mut self.write, value) else {
                    return Some(Err(RuntimeError::OutputError))
                };
            }
        }
        Some(Ok(()))
    }

    pub fn run_all(&mut self) -> Result<(), RuntimeError> {
        while let Some(result) = self.step() {
            result?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction as I, Program, Settings, VirtualMachine};

    fn assert_interpret(program: Program, input: &str, output: &str) {
        let mut buffer = Vec::new();
        let mut vm =
            VirtualMachine::new(program, Settings::default(), input.as_bytes(), &mut buffer);
        let res = vm.run_all();
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
        let mut vm = VirtualMachine::new_std(vec![I::MutPointer(-1)]);
        vm.run_all().unwrap();
        assert_eq!(vm.pointer, 29_999);
    }

    #[test]
    fn mut_cell_reaches_all_values() {
        let mut vm =
            VirtualMachine::new_std(vec![I::MutCell(127), I::SetCell(0), I::MutCell(-128)]);
        vm.step().unwrap().unwrap();
        assert_eq!(*vm.cell(), 127);
        vm.step().unwrap().unwrap();
        vm.step().unwrap().unwrap();
        assert_eq!(*vm.cell(), 128);
    }
}
