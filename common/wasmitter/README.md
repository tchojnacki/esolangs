# `wasmitter` 👷

[![docs](https://img.shields.io/badge/docs-passing-success)](https://tchojnacki.github.io/esolangs/wasmitter)

**wasmitter** (WASM emitter) is a crate for building and emitting WebAssembly modules programmatically.

## Examples
```Rust
use wasmitter::{Module, Instr, types::{I32, Mut}, instruction::ConstInstr};

fn main() {
    let mut module = Module::new();

    let print = module.import_func("console", "log", "$print", I32, ());

    let answer = module.global("$answer", Mut::Const, ConstInstr::I32Const(42));

    let main = module.func("$main", |scope| {
        vec![Instr::GlobalGet(answer), Instr::Call(print)]
    });

    module.export("_start", main);

    println!("{}", module.to_wat().expect("the module is invalid"));
}
```
