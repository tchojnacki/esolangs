use wasmitter::{Instr, MemArg, Module, Mutability, Nn, Sx, I32};

pub fn main() {
    let mut module = Module::new();

    let fd_read = module.import_func(
        "wasi_unstable",
        "fd_read",
        Some("$fd_read"),
        vec![I32, I32, I32, I32],
        I32,
    );
    let fd_write = module.import_func(
        "wasi_unstable",
        "fd_write",
        Some("$fd_write"),
        vec![I32, I32, I32, I32],
        I32,
    );

    let ptr = module.global(Some("$ptr"), Mutability::Mut, I32, Instr::I32Const(0));
    let memory = module.memory(None, 1, 1);

    let read_byte = module.func(Some("$read_byte"), |scope| {
        scope.add_result(I32);
        vec![
            Instr::I32Const(0),
            Instr::I32Const(30004),
            Instr::I32Const(1),
            Instr::I32Const(30000),
            Instr::Call(fd_read),
            Instr::Drop,
            Instr::I32Const(30012),
            Instr::ILoad(Nn::N32, MemArg::default()),
        ]
    });

    let write_byte = module.func(Some("$write_byte"), |scope| {
        let value = scope.add_param(I32);
        vec![
            Instr::I32Const(30024),
            Instr::LocalGet(value),
            Instr::IStore(Nn::N32, MemArg::default()),
            Instr::I32Const(1),
            Instr::I32Const(30016),
            Instr::I32Const(1),
            Instr::I32Const(30000),
            Instr::Call(fd_write),
            Instr::Drop,
        ]
    });

    let mut_pointer = module.func(Some("$mut_pointer"), |scope| {
        let offset = scope.add_param(I32);
        vec![
            Instr::GlobalGet(ptr),
            Instr::LocalGet(offset),
            Instr::IAdd(Nn::N32),
            Instr::I32Const(30000),
            Instr::IRem(Nn::N32, Sx::U),
            Instr::GlobalSet(ptr),
        ]
    });

    let _mut_cell = module.func(Some("$mut_cell"), |scope| {
        let change = scope.add_param(I32);
        vec![
            Instr::GlobalGet(ptr),
            Instr::GlobalGet(ptr),
            Instr::ILoad8(Nn::N32, Sx::U, MemArg::default()),
            Instr::LocalGet(change),
            Instr::IAdd(Nn::N32),
            Instr::I32Const(256),
            Instr::IRem(Nn::N32, Sx::U),
            Instr::IStore8(Nn::N32, MemArg::default()),
        ]
    });

    let set_cell = module.func(Some("$set_cell"), |scope| {
        let value = scope.add_param(I32);
        vec![
            Instr::GlobalGet(ptr),
            Instr::LocalGet(value),
            Instr::IStore8(Nn::N32, MemArg::default()),
        ]
    });

    let input = module.func(Some("$input"), |_| {
        vec![Instr::Call(read_byte), Instr::Call(set_cell)]
    });

    let output = module.func(Some("$output"), |_| {
        vec![
            Instr::GlobalGet(ptr),
            Instr::ILoad8(Nn::N32, Sx::U, MemArg::default()),
            Instr::Call(write_byte),
        ]
    });

    let main = module.func(Some("$main"), |_| {
        vec![
            Instr::I32Const(30004),
            Instr::I32Const(30012),
            Instr::IStore(Nn::N32, MemArg::default()),
            Instr::I32Const(30008),
            Instr::I32Const(1),
            Instr::IStore(Nn::N32, MemArg::default()),
            Instr::I32Const(30016),
            Instr::I32Const(30024),
            Instr::IStore(Nn::N32, MemArg::default()),
            Instr::I32Const(30020),
            Instr::I32Const(1),
            Instr::IStore(Nn::N32, MemArg::default()),
            Instr::Call(input),
            Instr::I32Const(1),
            Instr::Call(mut_pointer),
            Instr::Call(input),
            Instr::Call(output),
            Instr::I32Const(-1i32 as u32),
            Instr::Call(mut_pointer),
            Instr::Call(output),
        ]
    });

    module.export("memory", memory);
    module.export("_start", main);

    // println!("{:#?}", module);

    println!("{}", module.to_wat());
}
