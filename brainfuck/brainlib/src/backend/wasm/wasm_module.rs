use std::io::{self, Write};

use wasmitter::{BlockType, Id, Instr as WI, MemArg, Module, Mutability, Nn, Sx, I32};

use crate::backend::{
    common::{Instruction as CI, Program, Settings},
    wasm::WasmTarget,
};

pub struct WasmModule(Module);

impl WasmModule {
    pub fn compile_from(program: &Program, target: WasmTarget, settings: &Settings) -> Self {
        let mut module = Module::new();

        let pages = target.required_pages(settings);
        let (read_byte, write_byte) = target.inject_io_funcs(&mut module, settings);

        let ptr = module.global("$ptr", Mutability::Mut, I32, WI::I32Const(0));
        let memory = module.memory(Id::none(), pages, pages);

        let mut_pointer = module.func("$mut_pointer", |scope| {
            let offset = scope.add_param(I32);
            vec![
                WI::GlobalGet(ptr),
                WI::LocalGet(offset),
                WI::IAdd(Nn::N32),
                WI::I32Const(settings.tape_length()),
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
                WI::I32Const(0xFF),
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

        let input = module.func("$input", |_| {
            vec![
                WI::Call(read_byte),
                WI::I32Const(0xFF),
                WI::IAnd(Nn::N32),
                WI::Call(set_cell),
            ]
        });

        let output = module.func("$output", |_| {
            vec![
                WI::GlobalGet(ptr),
                WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                WI::Call(write_byte),
            ]
        });

        let mut stack = vec![Vec::new()];

        for instr in program.0.iter() {
            let current = stack.last_mut().expect("unexpected stack underflow");

            match instr {
                CI::MutPointer(change) => current.append(&mut vec![
                    WI::I32Const(*change as u32),
                    WI::Call(mut_pointer),
                ]),
                CI::MutCell(change) =>
                    current.append(&mut vec![WI::I32Const(*change as u32), WI::Call(mut_cell)]),
                CI::SetCell(value) =>
                    current.append(&mut vec![WI::I32Const(*value as u32), WI::Call(set_cell)]),
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
            [target.main_prelude(settings), body].concat()
        });

        module.export("memory", memory);
        module.export("_start", main);

        Self(module)
    }

    pub fn emit_wat(&self, mut write: impl Write) -> io::Result<()> {
        write.write_all(self.0.to_wat().as_bytes())
    }
}
