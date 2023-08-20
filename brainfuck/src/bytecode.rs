use crate::{parser::Tree, Node};

pub type Program = Vec<Instruction>;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    MutPointer(i32),
    MutCell(i8),
    SetCell(u8),
    RelJumpRightZero(u32),
    RelJumpLeftNotZero(u32),
    Input,
    Output,
}

pub fn generate(ast: &Tree) -> Program {
    let mut result = Vec::new();
    for node in ast.iter() {
        use {Instruction as I, Node as N};
        match node {
            N::Right => result.push(I::MutPointer(1)),
            N::Left => result.push(I::MutPointer(-1)),
            N::Increment => result.push(I::MutCell(1)),
            N::Decrement => result.push(I::MutCell(-1)),
            N::Output => result.push(I::Output),
            N::Input => result.push(I::Input),
            N::Loop(subtree) => {
                let mut subcode = generate(subtree);
                let jump = subcode.len() as u32 + 1;

                result.push(I::RelJumpRightZero(jump));
                result.append(&mut subcode);
                result.push(I::RelJumpLeftNotZero(jump));
            }
        };
    }
    result
}

// TODO: tests
