use crate::{
    backend::instruction::{Instruction, Program},
    frontend::ast::{Node, Tree},
};

#[must_use]
pub fn emit(ast: &Tree) -> Program {
    let mut result = Vec::new();
    for node in ast.iter() {
        use self::{Instruction as I, Node as N};
        match node {
            N::Right => result.push(I::MutPointer(1)),
            N::Left => result.push(I::MutPointer(-1)),
            N::Increment => result.push(I::MutCell(1)),
            N::Decrement => result.push(I::MutCell(-1)),
            N::Output => result.push(I::Output),
            N::Input => result.push(I::Input),
            N::Loop(subtree) => {
                let mut subcode = emit(subtree);
                let jump = subcode.len() as u32 + 1;

                result.push(I::JumpRightZ(jump));
                result.append(&mut subcode);
                result.push(I::JumpLeftNz(jump));
            },
            N::Breakpoint(pos) => result.push(I::Breakpoint(*pos as u32)),
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{emit, Instruction as I, Node as N};

    #[test]
    fn emits_correct_loop_offsets() {
        assert_eq!(
            emit(&vec![N::Loop(vec![N::Decrement].into_boxed_slice())].into_boxed_slice()),
            vec![I::JumpRightZ(2), I::MutCell(-1), I::JumpLeftNz(2)]
        );
    }

    #[test]
    fn handles_nested_loops() {
        assert_eq!(
            emit(
                &vec![N::Loop(
                    vec![N::Loop(vec![N::Decrement].into_boxed_slice())].into_boxed_slice()
                )]
                .into_boxed_slice()
            ),
            vec![
                I::JumpRightZ(4),
                I::JumpRightZ(2),
                I::MutCell(-1),
                I::JumpLeftNz(2),
                I::JumpLeftNz(4)
            ]
        );
    }
}
