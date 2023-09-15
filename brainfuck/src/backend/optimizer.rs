use std::collections::VecDeque;

use self::builder::Builder;
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
    if settings.strict() {
        return input;
    }

    let mut input = input.into_iter().peekable();
    let mut builder = Builder::with_capacity(input.len());

    while let Some(instr) = input.next() {
        match instr {
            I::MutPointer(mut value) => {
                builder.omit(1);
                while let Some(I::MutPointer(_)) = input.peek() {
                    builder.omit(1);
                    value = (value + input.next().unwrap().unwrap_mut_pointer())
                        .rem_euclid(settings.tape_length() as i32);
                }
                builder.include(I::MutPointer(value));
            }
            I::MutCell(mut value) => {
                builder.omit(1);
                while let Some(I::MutCell(_)) = input.peek() {
                    builder.omit(1);
                    value = value.wrapping_add(input.next().unwrap().unwrap_mut_cell());
                }
                builder.include(I::MutCell(value));
            }
            other => builder.preserve(other),
        }
    }

    builder.build()
}

fn create_sets(input: Program, settings: &Settings) -> Program {
    let mut builder = Builder::with_capacity(input.len());
    let mut queue = VecDeque::with_capacity(3);

    for instr in input {
        if queue.len() == 3 {
            builder.preserve(queue.pop_front().unwrap());
        }
        queue.push_back(instr);

        if queue.len() == 3 {
            if let (I::JumpRightZ(_), I::MutCell(value), I::JumpLeftNz(_)) =
                (queue[0], queue[1], queue[2])
            {
                queue.clear();
                builder.omit(3);
                if value == 1 && settings.strict() {
                    return builder.overflow();
                } else if value == -1 || value == 1 {
                    builder.include(I::SetCell(0));
                }
            }
        }
    }

    while let Some(instr) = queue.pop_front() {
        builder.preserve(instr);
    }

    builder.build()
}

mod builder {
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

    pub struct Builder {
        jumps: Vec<JumpEntry>,
        result: Vec<I>,
    }

    impl Builder {
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                jumps: Vec::new(),
                result: Vec::with_capacity(capacity),
            }
        }

        pub fn preserve(&mut self, instr: I) {
            self.result.push(instr);
            let last = self.result.len() - 1;
            match instr {
                I::JumpRightZ(jump) => self.jumps.push(JumpEntry {
                    index: last,
                    jump,
                    changed: 0,
                }),
                I::JumpLeftNz(_) => {
                    let entry = self.jumps.pop().unwrap();
                    self.result[entry.index] = I::JumpRightZ(entry.new_jump());
                    self.result[last] = I::JumpLeftNz(entry.new_jump());
                }
                _ => (),
            }
        }

        pub fn include(&mut self, instr: I) {
            self.result.push(instr);
            for entry in &mut self.jumps {
                entry.changed += 1;
            }
        }

        pub fn omit(&mut self, count: usize) {
            for entry in &mut self.jumps {
                entry.changed -= count as i32;
            }
        }

        pub fn build(self) -> Program {
            self.result
        }

        pub fn overflow(mut self) -> Program {
            self.result.push(I::SetCell(255));
            self.result.push(I::MutCell(1));
            self.result
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
                    I::JumpRightZ(5),
                    I::MutCell(3),
                    I::MutCell(5),
                    I::MutCell(-2),
                    I::MutPointer(1),
                    I::JumpLeftNz(5)
                ],
                &Settings::default()
            ),
            vec![
                I::JumpRightZ(3),
                I::MutCell(6),
                I::MutPointer(1),
                I::JumpLeftNz(3)
            ]
        )
    }

    #[test]
    fn optimize_creates_sets() {
        assert_eq!(
            optimize(
                vec![
                    I::MutPointer(5),
                    I::JumpRightZ(3),
                    I::MutCell(3),
                    I::MutCell(-4),
                    I::JumpLeftNz(3),
                    I::MutPointer(-5)
                ],
                &Settings::default()
            ),
            vec![I::MutPointer(5), I::SetCell(0), I::MutPointer(-5)]
        )
    }

    #[test]
    fn optimizes_creates_sets_in_strict() {
        assert_eq!(
            optimize(
                vec![I::JumpRightZ(3), I::MutCell(-1), I::JumpLeftNz(3)],
                &Settings::default_strict()
            ),
            vec![I::SetCell(0)]
        )
    }

    #[test]
    fn optimizes_preserves_overflow_in_strict() {
        assert_eq!(
            optimize(
                vec![
                    I::JumpRightZ(3),
                    I::MutCell(1),
                    I::JumpLeftNz(3),
                    I::MutPointer(3)
                ],
                &Settings::default_strict()
            ),
            vec![I::SetCell(255), I::MutCell(1)]
        )
    }
}
