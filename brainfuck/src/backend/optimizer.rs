use std::collections::VecDeque;

use self::jumps::JumpStack;
use crate::{
    backend::instruction::{Instruction as I, Program},
    Settings,
};

#[must_use]
pub fn optimize(program: Program, settings: &Settings) -> Program {
    let program = merge_muts(program, settings);
    create_sets(program, settings)
}

fn merge_muts(input: Program, settings: &Settings) -> Program {
    let mut input = input.into_iter().peekable();
    let mut jump_stack = JumpStack::default();
    let mut result = Vec::with_capacity(input.len());

    while let Some(instr) = input.next() {
        match instr {
            I::MutPointer(mut value) => {
                while let Some(I::MutPointer(next_value)) = input.peek().copied() {
                    let _ = input.next().unwrap();
                    value = (value + next_value).rem_euclid(settings.tape_length() as i32);
                    jump_stack.change(-1);
                }
                result.push(I::MutPointer(value));
            }
            I::MutCell(mut value) => {
                while let Some(I::MutCell(next_value)) = input.peek().copied() {
                    let _ = input.next().unwrap();
                    value = value.wrapping_add(next_value);
                    jump_stack.change(-1);
                }
                result.push(I::MutCell(value));
            }
            other => jump_stack.visit(&mut result, other),
        }
    }

    result
}

fn create_sets(input: Program, settings: &Settings) -> Program {
    let mut jump_stack = JumpStack::default();
    let mut result = Vec::with_capacity(input.len());
    let mut queue = VecDeque::with_capacity(3);

    for instr in input {
        if queue.len() == 3 {
            jump_stack.visit(&mut result, queue.pop_front().unwrap());
        }
        queue.push_back(instr);

        if queue.len() == 3 {
            if let (I::RelJumpRightZero(_), I::MutCell(value), I::RelJumpLeftNotZero(_)) =
                (queue[0], queue[1], queue[2])
            {
                if value == 1 && settings.strict() {
                    queue.clear();
                    result.push(I::SetCell(255));
                    result.push(I::MutCell(1));
                    jump_stack.change(-1);
                } else if value == -1 || value == 1 {
                    queue.clear();
                    result.push(I::SetCell(0));
                    jump_stack.change(-2);
                }
            }
        }
    }

    while let Some(instr) = queue.pop_front() {
        jump_stack.visit(&mut result, instr);
    }

    result
}

mod jumps {
    use super::{Program, I};

    struct JumpEntry {
        index: usize,
        jump: u32,
        changed: i32,
    }

    impl JumpEntry {
        pub const fn new_jump(&self) -> u32 {
            (self.jump as i32 + self.changed) as u32
        }
    }

    #[derive(Default)]
    pub struct JumpStack(Vec<JumpEntry>);

    impl JumpStack {
        pub fn visit(&mut self, program: &mut Program, instr: I) {
            program.push(instr);
            let last = program.len() - 1;
            match instr {
                I::RelJumpRightZero(jump) => self.0.push(JumpEntry {
                    index: program.len() - 1,
                    jump,
                    changed: 0,
                }),
                I::RelJumpLeftNotZero(_) => {
                    let entry = self.0.pop().unwrap();
                    program[entry.index] = I::RelJumpRightZero(entry.new_jump());
                    program[last] = I::RelJumpLeftNotZero(entry.new_jump());
                }
                _ => (),
            }
        }

        pub fn change(&mut self, by: i32) {
            for entry in &mut self.0 {
                entry.changed += by;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{optimize, Settings, I};

    #[test]
    fn optimize_combines_mut_cells() {
        assert_eq!(
            optimize(
                vec![I::MutCell(3), I::MutCell(127), I::MutCell(-128)],
                &Settings::default(),
            ),
            vec![I::MutCell(2)]
        );

        assert_eq!(
            optimize(
                vec![
                    I::MutCell(127),
                    I::MutCell(1),
                    I::MutPointer(1),
                    I::MutCell(-13)
                ],
                &Settings::default(),
            ),
            vec![I::MutCell(-128), I::MutPointer(1), I::MutCell(-13)]
        );
    }

    #[test]
    fn optimize_correctly_edits_jumps() {
        assert_eq!(
            optimize(
                vec![
                    I::RelJumpRightZero(5),
                    I::MutCell(3),
                    I::MutCell(5),
                    I::MutCell(-2),
                    I::MutPointer(1),
                    I::RelJumpLeftNotZero(5)
                ],
                &Settings::default()
            ),
            vec![
                I::RelJumpRightZero(3),
                I::MutCell(6),
                I::MutPointer(1),
                I::RelJumpLeftNotZero(3)
            ]
        )
    }

    #[test]
    fn optimize_creates_sets() {
        assert_eq!(
            optimize(
                vec![
                    I::MutPointer(5),
                    I::RelJumpRightZero(3),
                    I::MutCell(3),
                    I::MutCell(-4),
                    I::RelJumpLeftNotZero(3),
                    I::MutPointer(-5)
                ],
                &Settings::default()
            ),
            vec![I::MutPointer(5), I::SetCell(0), I::MutPointer(-5)]
        )
    }

    #[test]
    fn optimizes_preserves_overflow_in_strict() {
        assert_eq!(
            optimize(
                vec![
                    I::RelJumpRightZero(3),
                    I::MutCell(4),
                    I::MutCell(-3),
                    I::RelJumpLeftNotZero(3),
                ],
                &Settings::default_strict()
            ),
            vec![I::SetCell(255), I::MutCell(1)]
        )
    }
}
