# `brainlib` 🧠

[![docs](https://img.shields.io/badge/docs-passing-success)](https://tchojnacki.github.io/esolangs/brainlib)

> [!NOTE]  
> Visit the [brainfuck](../#readme) directory to learn more about the language.

## Examples

```Rust
let source = "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.";
let program = Program::compile(source, &Settings::new()).expect("parse error");
Engine::new_std_default(program).run().expect("runtime error");
```

## Features

- Fully implemented: lexer, parser, bytecode generator, optimizer, interpreter.
- Ability to compile to WebAssembly.
- Covered by unit and property tests.
- No unsafe code.
