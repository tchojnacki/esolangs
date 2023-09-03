use crate::{
    backend::instruction::{Instruction, Program},
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
    read: R,
    write: W,
}

impl VirtualMachine<Stdin, Stdout> {
    pub fn new_std(program: Program) -> Self {
        Self::new(program, 30_000, stdin(), stdout())
    }
}

impl<R: Read, W: Write> VirtualMachine<R, W> {
    pub fn new(program: Program, tape_length: usize, read: R, write: W) -> Self {
        Self {
            program,
            pc: 0,
            pointer: 0,
            memory: vec![0; tape_length].into_boxed_slice(),
            read,
            write,
        }
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
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add_signed(offset);
            }
            I::SetCell(value) => {
                self.memory[self.pointer] = value;
            }
            I::RelJumpRightZero(offset) => {
                if self.memory[self.pointer] == 0 {
                    self.pc += offset as usize;
                }
            }
            I::RelJumpLeftNotZero(offset) => {
                if self.memory[self.pointer] != 0 {
                    self.pc -= offset as usize;
                }
            }
            I::Input => {
                match read_byte(&mut self.read) {
                    Some(value) => self.memory[self.pointer] = value,
                    None => return Some(Err(RuntimeError::InputError)),
                };
            }
            I::Output => {
                let Some(()) = write_byte(&mut self.write, self.memory[self.pointer]) else {
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
    use super::{Instruction as I, Program, VirtualMachine};

    fn assert_interpret(program: Program, input: &str, output: &str) {
        let mut buffer = Vec::new();
        let mut vm = VirtualMachine::new(program, 30_000, input.as_bytes(), &mut buffer);
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

    // TODO: better tests
}
