use std::io::{self, Write};

use wasmitter::{
    indices::{FuncIdx, GlobalIdx},
    instruction::{BlockType, ConstInstr as CWI, Instr as WI, MemArg, Nn, Sx},
    module::Module,
    text::Id,
    types::Mut,
};

use crate::backend::{
    common::{Instruction as CI, Program, Settings},
    wasm::WasmTarget,
};

#[must_use]
pub struct WasmModule(Module);

impl WasmModule {
    pub fn compile_from(program: &Program, target: WasmTarget, settings: &Settings) -> Self {
        let mut module = Module::new();

        let pages = target.required_pages(settings);
        let (read_byte, write_byte) = target.inject_io_funcs(&mut module, settings);

        let ptr = module.global("$ptr", Mut::Var, CWI::I32Const(0));
        let memory = module.memory(Id::none(), (pages, pages));

        let mut stack = vec![Vec::new()];

        for instr in program.0.iter() {
            let current = stack.last_mut().expect("unexpected stack underflow");

            match instr {
                CI::MutPointer(change) => current.append(&mut mut_pointer(settings, ptr, *change)),
                CI::MutCell(change) => current.append(&mut mut_cell(settings, ptr, *change)),
                CI::SetCell(value) => current.append(&mut set_cell(ptr, *value)),
                CI::JumpRightZ(_) => stack.push(Vec::new()),
                CI::JumpLeftNz(_) => {
                    let body = stack.pop().expect("unexpected stack underflow");
                    let current = stack.last_mut().expect("unexpected stack underflow");

                    current.push(WI::Block(
                        BlockType::default(),
                        [
                            loop_header(ptr),
                            vec![WI::Loop(
                                BlockType::default(),
                                [body, loop_trailer(ptr)].concat(),
                            )],
                        ]
                        .concat(),
                    ))
                },
                CI::Input => current.append(&mut input(ptr, read_byte)),
                CI::Output => current.append(&mut output(ptr, write_byte)),
                CI::Breakpoint(_) => current.push(WI::Nop),
            }
        }

        let main = module.func("$main", |_| {
            let body = stack.pop().expect("unexpected stack underflow");
            assert!(stack.is_empty(), "unexpected stack overflow");
            [target.main_header(settings), body].concat()
        });

        module.export("memory", memory);
        module.export("_start", main);

        Self(module)
    }

    pub fn emit_wat(&self, mut write: impl Write) -> io::Result<()> {
        write.write_all(self.0.to_wat().expect("internal error").as_bytes())
    }
}

#[must_use]
fn mut_pointer(settings: &Settings, ptr: GlobalIdx, change: i32) -> Vec<WI> {
    [
        vec![
            WI::GlobalGet(ptr),
            WI::I32Const(change as u32),
            WI::IAdd(Nn::N32),
            WI::GlobalSet(ptr),
        ],
        if settings.strict() {
            vec![
                WI::Block(
                    BlockType::default(),
                    vec![
                        WI::GlobalGet(ptr),
                        WI::I32Const(0),
                        WI::IGe(Nn::N32, Sx::S),
                        WI::BrIf(0.into()),
                        WI::Unreachable,
                    ],
                ),
                WI::Block(
                    BlockType::default(),
                    vec![
                        WI::GlobalGet(ptr),
                        WI::I32Const(settings.tape_length()),
                        WI::ILt(Nn::N32, Sx::S),
                        WI::BrIf(0.into()),
                        WI::Unreachable,
                    ],
                ),
            ]
        } else {
            vec![
                WI::GlobalGet(ptr),
                WI::I32Const(settings.tape_length()),
                WI::IRem(Nn::N32, Sx::U),
                WI::GlobalSet(ptr),
            ]
        },
    ]
    .concat()
}

#[must_use]
fn mut_cell(settings: &Settings, ptr: GlobalIdx, change: i8) -> Vec<WI> {
    [
        vec![
            WI::GlobalGet(ptr),
            WI::GlobalGet(ptr),
            WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
            WI::I32Const(change as u32),
            WI::IAdd(Nn::N32),
            WI::IStore8(Nn::N32, MemArg::default()),
        ],
        if settings.strict() {
            vec![WI::Block(
                BlockType::default(),
                vec![
                    WI::GlobalGet(ptr),
                    WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                    WI::I32Const(!0xFF),
                    WI::IAnd(Nn::N32),
                    WI::IEqz(Nn::N32),
                    WI::BrIf(0.into()),
                    WI::Unreachable,
                ],
            )]
        } else {
            vec![
                WI::GlobalGet(ptr),
                WI::GlobalGet(ptr),
                WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
                WI::I32Const(0xFF),
                WI::IAnd(Nn::N32),
                WI::IStore8(Nn::N32, MemArg::default()),
            ]
        },
    ]
    .concat()
}

#[must_use]
fn set_cell(ptr: GlobalIdx, value: u8) -> Vec<WI> {
    vec![
        WI::GlobalGet(ptr),
        WI::I32Const(value as u32),
        WI::IStore8(Nn::N32, MemArg::default()),
    ]
}

#[must_use]
fn input(ptr: GlobalIdx, read_byte: FuncIdx) -> Vec<WI> {
    vec![
        WI::GlobalGet(ptr),
        WI::Call(read_byte),
        WI::I32Const(0xFF),
        WI::IAnd(Nn::N32),
        WI::IStore8(Nn::N32, MemArg::default()),
    ]
}

#[must_use]
fn output(ptr: GlobalIdx, write_byte: FuncIdx) -> Vec<WI> {
    vec![
        WI::GlobalGet(ptr),
        WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
        WI::Call(write_byte),
    ]
}

#[must_use]
fn loop_header(ptr: GlobalIdx) -> Vec<WI> {
    vec![
        WI::GlobalGet(ptr),
        WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
        WI::IEqz(Nn::N32),
        WI::BrIf(0.into()),
    ]
}

#[must_use]
fn loop_trailer(ptr: GlobalIdx) -> Vec<WI> {
    vec![
        WI::GlobalGet(ptr),
        WI::ILoad8(Nn::N32, Sx::U, MemArg::default()),
        WI::BrIf(0.into()),
    ]
}
