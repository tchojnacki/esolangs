# Brainfuck

> Brainfuck is an esoteric programming language created in 1993 by Urban Müller. Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers [^1].

## Features

- Fully implemented lexer, parser, bytecode generator, optimizer and interpreter.
- Can be used as a binary or library.
- Covered by unit and integration tests.
- Reading code from file, as an argument or from stdin.
- Conventions:
  - Starting cell index: 0
  - Cell size: 8 bit unsigned
  - Cell overflow: wrapping (customizable)
  - Tape length: 30 000 (customizable)
  - Tape overflow: wrapping (customizable)

## Examples

### Binary

```
bf --help
bf -f specs/mandelbrot.code.bf
bf -c "++>+++++[<+>-]++++++++[<++++++>-]<."
```

### Library

```Rust
let program = compile_debug("+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.").expect("Parse error!");
VirtualMachine::new_std_default(program).run().expect("Runtime error!");
```

## TODO

- Built-in [debugger](https://esolangs.org/wiki/Brainfuck#Extensions).
- Other targets (WASM, JIT).
- Documentation for public API.

[^1]: [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)
