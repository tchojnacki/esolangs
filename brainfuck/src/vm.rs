use std::io::{Read, Write};

use crate::instruction::Instruction;

pub struct VirtualMachine<R: Read, W: Write> {
    pointer: usize,
    data: Box<[u8]>,
    read: R,
    write: W,
}

impl<R: Read, W: Write> VirtualMachine<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self {
            pointer: 0,
            data: Box::new([0; 30_000]),
            read,
            write,
        }
    }

    pub fn interpret(&mut self, code: &[Instruction]) {
        for instruction in code {
            use Instruction::*;
            match instruction {
                Right => self.pointer += 1,
                Left => self.pointer -= 1,
                Increment => self.data[self.pointer] += 1,
                Decrement => self.data[self.pointer] -= 1,
                Output => {
                    self.write.write_all(&[self.data[self.pointer]]).unwrap();
                }
                Input => {
                    let mut buffer = [0];
                    self.read.read_exact(&mut buffer).unwrap();
                    self.data[self.pointer] = buffer[0];
                }
                Loop(body) => {
                    while self.data[self.pointer] != 0 {
                        self.interpret(body);
                    }
                }
            }
        }
    }
}
