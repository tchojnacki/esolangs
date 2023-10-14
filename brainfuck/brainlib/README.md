# `brainlib` 🧠

> [!NOTE]  
> Visit the [brainfuck](../#readme) directory to learn more about the language.

## Examples

```Rust
let source = "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.";
let program = Program::compile(source, &Settings::default()).expect("Parse error!");
VirtualMachine::new_std_default(program).run().expect("Runtime error!");
```

## Features

- Fully implemented: lexer, parser, bytecode generator, optimizer, interpreter.
- Covered by unit and property tests.
- No runtime dependencies.

## TODO

- Other targets (WASM, JIT).
- Documentation for public API.
- Better encapsulation.
