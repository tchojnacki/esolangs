use std::{collections::VecDeque, mem};

use self::builder::Builder;
use crate::backend::common::{
    instruction::{Instruction as I, Program},
    settings::Settings,
};

#[must_use]
pub fn optimize(program: Program, settings: &Settings) -> Program {
    if settings.debug() {
        return program;
    }

    let program = remove_breakpoints(program);
    let program = merge_muts(program, settings);
    let program = create_sets(program, settings);
    reduce_cell_chains(program, settings)
}

fn remove_breakpoints(input: Program) -> Program {
    let mut builder = Builder::with_capacity(input.len());
    for instr in input {
        if let I::Breakpoint(_) = instr {
            builder.omit(1);
        } else {
            builder.preserve(instr);
        }
    }
    builder.build()
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
            },
            I::MutCell(mut value) => {
                builder.omit(1);
                while let Some(I::MutCell(_)) = input.peek() {
                    builder.omit(1);
                    value = value.wrapping_add(input.next().unwrap().unwrap_mut_cell());
                }
                builder.include(I::MutCell(value));
            },
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
                if value == 1 && settings.strict() {
                    builder.preserve(queue[0]);
                    builder.preserve(queue[1]);
                    builder.preserve(queue[2]);
                } else if value == -1 || value == 1 {
                    builder.omit(3);
                    builder.include(I::SetCell(0));
                }
                queue.clear();
            }
        }
    }

    while let Some(instr) = queue.pop_front() {
        builder.preserve(instr);
    }

    builder.build()
}

fn reduce_cell_chains(input: Program, settings: &Settings) -> Program {
    let mut builder = Builder::with_capacity(input.len());
    let mut chain = (None, Vec::new());

    let include_all_changes = |builder: &mut Builder, changes: &[i8]| {
        for change in changes {
            if *change != 0 {
                builder.include(I::MutCell(*change));
            }
        }
    };

    let change_value = |mut value: u8, changes: &[i8]| {
        for change in changes {
            if let Some(new) = settings.mut_cell(value, *change) {
                value = new;
            } else {
                return Err(());
            }
        }
        Ok(value)
    };

    macro_rules! finish_chain {
        () => {
            let (set, changes) = mem::take(&mut chain);
            match set {
                Some(value) => match change_value(value, &changes) {
                    Ok(new) => builder.include(I::SetCell(new)),
                    Err(_) => return builder.overflow(),
                },
                None =>
                    if settings.strict() {
                        include_all_changes(&mut builder, &changes);
                    } else {
                        let value = changes
                            .iter()
                            .fold(0i8, |acc, &change| acc.wrapping_add(change));
                        if value != 0 {
                            builder.include(I::MutCell(value));
                        }
                    },
            };
        };
    }

    for instr in input {
        match instr {
            I::SetCell(value) => {
                builder.omit(1);
                if settings.strict() {
                    match chain.0 {
                        Some(value) =>
                            if change_value(value, &chain.1).is_err() {
                                return builder.overflow();
                            },
                        None => include_all_changes(&mut builder, &chain.1),
                    }
                }
                chain = (Some(value), Vec::new());
            },
            I::MutCell(change) => {
                builder.omit(1);
                chain.1.push(change);
            },
            other => {
                finish_chain!();
                builder.preserve(other);
            },
        }
    }

    finish_chain!();

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
                },
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
    use std::ops::RangeBounds;

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use test_case::test_case;

    use super::{optimize, Program, Settings, I};
    use crate::interpreter::VirtualMachine;

    fn rand_range<T, R>(gen: &mut Gen, range: R) -> T
    where
        T: Copy,
        R: RangeBounds<T> + Iterator<Item = T>,
    {
        *gen.choose(&range.collect::<Vec<_>>()).unwrap()
    }

    #[derive(Clone, Debug)]
    struct SimpleProgram(Program);

    impl Arbitrary for SimpleProgram {
        fn arbitrary(gen: &mut Gen) -> Self {
            let len = rand_range(gen, 0..256);
            Self(
                (0..len)
                    .map(|_| {
                        let mut_cell = I::MutCell(rand_range(gen, -128..=127));
                        let mut_pointer = I::MutPointer(rand_range(gen, -100..=100));
                        let set_cell = I::SetCell(rand_range(gen, 0..=255));
                        let breakpoint = I::Breakpoint(rand_range(gen, 0..len));
                        *gen.choose(&[mut_cell, mut_pointer, set_cell, breakpoint])
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }

    impl Arbitrary for Settings {
        fn arbitrary(g: &mut Gen) -> Self {
            let len = *g.choose(&[3, 10, 100, 256, 1024]).unwrap();
            g.choose(&[
                Settings::try_new(len, false, false).unwrap(),
                Settings::try_new(len, true, false).unwrap(),
            ])
            .unwrap()
            .clone()
        }
    }

    #[test]
    fn merges_mut_cells_without_strict() {
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
    fn does_not_merge_mut_cells_with_strict() {
        assert_eq!(
            optimize(
                vec![I::SetCell(250), I::MutCell(10), I::MutCell(-9)],
                &Settings::default_strict()
            ),
            vec![I::SetCell(255), I::MutCell(1)]
        )
    }

    #[test_case(Settings::default(); "without strict")]
    #[test_case(Settings::default_strict(); "with strict")]
    fn edits_jumps(settings: Settings) {
        assert_eq!(
            optimize(
                vec![
                    I::JumpRightZ(5),
                    I::SetCell(1),
                    I::SetCell(2),
                    I::SetCell(3),
                    I::MutPointer(1),
                    I::JumpLeftNz(5)
                ],
                &settings
            ),
            vec![
                I::JumpRightZ(3),
                I::SetCell(3),
                I::MutPointer(1),
                I::JumpLeftNz(3)
            ]
        )
    }

    #[test]
    fn merges_and_creates_sets_without_stricts() {
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

    #[test_case(Settings::default(); "without strict")]
    #[test_case(Settings::default_strict(); "with strict")]
    fn creates_sets(settings: Settings) {
        assert_eq!(
            optimize(
                vec![I::JumpRightZ(3), I::MutCell(-1), I::JumpLeftNz(3)],
                &settings
            ),
            vec![I::SetCell(0)]
        )
    }

    #[test]
    fn preserves_loop_overflow_with_strict() {
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
            vec![
                I::JumpRightZ(3),
                I::MutCell(1),
                I::JumpLeftNz(3),
                I::MutPointer(3)
            ]
        )
    }

    #[test]
    fn removes_leading_muts_without_strict() {
        assert_eq!(
            optimize(
                vec![
                    I::MutCell(5),
                    I::MutCell(-3),
                    I::SetCell(10),
                    I::MutCell(-2)
                ],
                &Settings::default()
            ),
            vec![I::SetCell(8)]
        )
    }

    #[test]
    fn does_not_remove_leading_muts_with_strict() {
        assert_eq!(
            optimize(
                vec![
                    I::MutCell(5),
                    I::MutCell(-3),
                    I::SetCell(10),
                    I::MutCell(-2)
                ],
                &Settings::default_strict()
            ),
            vec![I::MutCell(5), I::MutCell(-3), I::SetCell(8)]
        )
    }

    #[test]
    fn removes_instructions_after_overflow() {
        assert_eq!(
            optimize(
                vec![
                    I::MutPointer(-3),
                    I::SetCell(200),
                    I::MutCell(100),
                    I::MutPointer(3),
                    I::JumpRightZ(2),
                    I::MutCell(-1),
                    I::JumpLeftNz(2),
                ],
                &Settings::default_strict()
            ),
            vec![I::MutPointer(-3), I::SetCell(255), I::MutCell(1)]
        );
    }

    #[quickcheck]
    fn reduces_instruction_count(simple_program: SimpleProgram, settings: Settings) -> bool {
        let SimpleProgram(program) = simple_program;
        let optimized = optimize(program.clone(), &settings);
        program.len() >= optimized.len()
    }

    #[quickcheck]
    fn creates_equivalent_code(program: SimpleProgram, settings: Settings) -> bool {
        let before = program.0;
        let mut before_vm = VirtualMachine::new_std(before.clone(), settings.clone());
        let before_res = before_vm.run();

        let after = optimize(before, &settings);
        let mut after_vm = VirtualMachine::new_std(after, settings);
        let after_res = after_vm.run();

        match before_res.is_ok() {
            true => after_res.is_ok() && before_vm.memory() == after_vm.memory(),
            false => after_res.is_err(),
        }
    }

    #[quickcheck]
    fn removes_all_breakpoints(simple_program: SimpleProgram, settings: Settings) -> bool {
        let SimpleProgram(program) = simple_program;
        let out = optimize(program, &settings);
        !out.iter().any(|i| matches!(i, I::Breakpoint(_)))
    }
}
