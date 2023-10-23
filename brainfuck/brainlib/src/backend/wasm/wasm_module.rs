use std::io::{self, Write};

use wasmitter::{BlockType, Id, Instr as WI, MemArg, Module, Mutability, Nn, Sx, I32};

use crate::backend::common::{Instruction as CI, Program};

pub struct WasmModule(Module);

impl WasmModule {
    pub fn compile_from(program: Program) -> Self {
        let mut module = Module::new();

        let fd_read = module.import_func(
            "wasi_unstable",
            "fd_read",
            "$fd_read",
            vec![I32, I32, I32, I32],
            I32,
        );

        let fd_write = module.import_func(
            "wasi_unstable",
            "fd_write",
            "$fd_write",
            vec![I32, I32, I32, I32],
            I32,
        );

        let read_byte = module.func("$read_byte", |scope| {
            scope.add_result(I32);
            vec![
                WI::I32Const(0),
                WI::I32Const(30004),
                WI::I32Const(1),
                WI::I32Const(30000),
                WI::Call(fd_read),
                WI::Drop,
                WI::I32Const(30012),
                WI::I32Load(MemArg::default()),
            ]
        });

        let write_byte = module.func("$write_byte", |scope| {
            let value = scope.add_param(I32);
            vec![
                WI::I32Const(30024),
                WI::LocalGet(value),
                WI::I32Store(MemArg::default()),
                WI::I32Const(1),
                WI::I32Const(30016),
                WI::I32Const(1),
                WI::I32Const(30000),
                WI::Call(fd_write),
                WI::Drop,
            ]
        });

        let ptr = module.global("$ptr", Mutability::Mut, I32, WI::I32Const(0));
        let memory = module.memory(Id::none(), 1, 1);

        let mut_pointer = module.func("$mut_pointer", |scope| {
            let offset = scope.add_param(I32);
            vec![
                WI::GlobalGet(ptr),
                WI::LocalGet(offset),
                WI::IAdd(Nn::N32),
                WI::I32Const(30000),
                WI::IRem(Nn::N32, Sx::U),
                WI::GlobalSet(ptr),
            ]
        });

        let mut_cell = module.func("$mut_cell", |scope| {
            let change = scope.add_param(I32);
            vec![
                WI::GlobalGet(ptr),
                WI::GlobalGet(ptr),
                WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                WI::LocalGet(change),
                WI::IAdd(Nn::N32),
                WI::I32Const(255),
                WI::IAnd(Nn::N32),
                WI::IStore8(Nn::N32, MemArg::default()),
            ]
        });

        let set_cell = module.func("$set_cell", |scope| {
            let value = scope.add_param(I32);
            vec![
                WI::GlobalGet(ptr),
                WI::LocalGet(value),
                WI::IStore8(Nn::N32, MemArg::default()),
            ]
        });

        let input = module.func("$input", |_| vec![WI::Call(read_byte), WI::Call(set_cell)]);

        let output = module.func("$output", |_| {
            vec![
                WI::GlobalGet(ptr),
                WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                WI::Call(write_byte),
            ]
        });

        let mut stack = vec![Vec::new()];

        for instr in program.0.into_iter() {
            let current = stack.last_mut().expect("unexpected stack underflow");

            match instr {
                CI::MutPointer(change) => current.append(&mut vec![
                    WI::I32Const(change as u32),
                    WI::Call(mut_pointer),
                ]),
                CI::MutCell(change) =>
                    current.append(&mut vec![WI::I32Const(change as u32), WI::Call(mut_cell)]),
                CI::SetCell(value) =>
                    current.append(&mut vec![WI::I32Const(value as u32), WI::Call(set_cell)]),
                CI::JumpRightZ(_) => stack.push(Vec::new()),
                CI::JumpLeftNz(_) => {
                    let body = stack.pop().expect("unexpected stack underflow");
                    let current = stack.last_mut().expect("unexpected stack underflow");

                    current.push(WI::Block(
                        BlockType::default(),
                        [
                            vec![
                                WI::GlobalGet(ptr),
                                WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                                WI::IEqz(Nn::N32),
                                WI::BrIf(0.into()),
                            ],
                            vec![WI::Loop(
                                BlockType::default(),
                                [
                                    body,
                                    vec![
                                        WI::GlobalGet(ptr),
                                        WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                                        WI::I32Const(0),
                                        WI::INe(Nn::N32),
                                        WI::BrIf(0.into()),
                                    ],
                                ]
                                .concat()
                                .into(),
                            )],
                        ]
                        .concat()
                        .into(),
                    ))
                },
                CI::Input => current.push(WI::Call(input)),
                CI::Output => current.push(WI::Call(output)),
                CI::Breakpoint(_) => (),
            }
        }

        let main = module.func("$main", |_| {
            let body = stack.pop().expect("unexpected stack underflow");
            assert!(stack.is_empty(), "unexpected stack overflow");
            [
                vec![
                    WI::I32Const(30004),
                    WI::I32Const(30012),
                    WI::I32Store(MemArg::default()),
                    WI::I32Const(30008),
                    WI::I32Const(1),
                    WI::I32Store(MemArg::default()),
                    WI::I32Const(30016),
                    WI::I32Const(30024),
                    WI::I32Store(MemArg::default()),
                    WI::I32Const(30020),
                    WI::I32Const(1),
                    WI::I32Store(MemArg::default()),
                ],
                body,
            ]
            .concat()
        });

        module.export("memory", memory);
        module.export("_start", main);

        Self(module)
    }

    pub fn emit_wat(&self, mut write: impl Write) -> io::Result<()> {
        write.write_all(self.0.to_wat().as_bytes())
    }
}
