use crate::parser::Node;
use std::io::{Read, Write};

const TAPE_LENGTH: usize = 30_000;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InputError,
    OutputError,
}

pub struct VirtualMachine<R: Read, W: Write> {
    dp: usize, // data pointer
    data: Box<[u8]>,
    read: R,
    write: W,
}

impl<R: Read, W: Write> VirtualMachine<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self {
            dp: 0,
            data: Box::new([0; TAPE_LENGTH]),
            read,
            write,
        }
    }

    pub fn interpret(&mut self, code: &[Node]) -> Result<(), RuntimeError> {
        for instruction in code {
            use {Node::*, RuntimeError::*};
            match instruction {
                Right => self.dp = (self.dp + 1) % TAPE_LENGTH,
                Left => self.dp = self.dp.checked_sub(1).unwrap_or(TAPE_LENGTH - 1),
                Increment => self.data[self.dp] = self.data[self.dp].wrapping_add(1),
                Decrement => self.data[self.dp] = self.data[self.dp].wrapping_sub(1),
                Output => write_u8(&mut self.write, self.data[self.dp]).ok_or(OutputError)?,
                Input => self.data[self.dp] = read_u8(&mut self.read).ok_or(InputError)?,
                Loop(body) => {
                    while self.data[self.dp] != 0 {
                        self.interpret(body)?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn read_u8<R: Read>(read: &mut R) -> Option<u8> {
    let mut buffer = [0];
    read.read_exact(&mut buffer).ok()?;
    Some(buffer[0])
}

fn write_u8<W: Write>(write: &mut W, value: u8) -> Option<()> {
    write.write_all(&[value]).ok()
}

#[cfg(test)]
mod tests {
    use super::{Node as N, VirtualMachine};

    fn assert_interpret(code: &[N], input: &str, output: &str) {
        let mut buffer = Vec::new();
        let mut vm = VirtualMachine::new(input.as_bytes(), &mut buffer);
        let res = vm.interpret(code);
        assert_eq!(res, Ok(()));
        assert_eq!(buffer, output.as_bytes());
    }

    #[test]
    fn starts_with_zero_at_cell_zero() {
        assert_interpret(&[N::Output], "", "\0")
    }

    #[test]
    fn cat_copies_input() {
        assert_interpret(
            &[N::Input, N::Loop(Box::new([N::Output, N::Input]))],
            "Hello, world!\0",
            "Hello, world!",
        )
    }

    #[test]
    fn decrement_reverses_increment() {
        assert_interpret(
            &[
                N::Input,
                N::Increment,
                N::Decrement,
                N::Decrement,
                N::Increment,
                N::Output,
            ],
            "x",
            "x",
        )
    }

    #[test]
    fn left_reverses_right() {
        assert_interpret(
            &[N::Input, N::Left, N::Right, N::Right, N::Left, N::Output],
            "A",
            "A",
        )
    }

    #[test]
    fn loop_zeroes_cell() {
        assert_interpret(
            &[N::Input, N::Loop(Box::new([N::Decrement])), N::Output],
            "X",
            "\0",
        )
    }
}
