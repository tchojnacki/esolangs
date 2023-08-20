# Brainfuck

> Brainfuck is an esoteric programming language created in 1993 by Urban Müller. Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers [^1].

## Features

- Fully implemented lexer, parser and interpreter.
- Can be used as a binary or library.
- Unit and integration test coverage.
- Implementation choices:
  - Cell size: 8 bit unsigned (with wrapping)
  - Cell count: 30 000 (with wrapping)
  - Starting cell index: 0

## Examples

### Binary

```
brainfuck --help
brainfuck tests/spec/hello-normal.code.bf
brainfuck <(echo "++>+++++[<+>-]++++++++[<++++++>-]<.")
```

### Library

```Rust
let source = "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.";
let tokens = tokenize(source.chars());
let code = parse(tokens).unwrap();
VirtualMachine::new(stdin(), stdout()).interpret(&code).unwrap();
```

## TODO

- Customizable wrapping.
- Customizable tape length.
- Passing [code through stdin](https://esolangs.org/wiki/Brainfuck#Extensions).
- Built-in [debugger](https://esolangs.org/wiki/Brainfuck#Extensions).
- Built-in code optimizer.
- Bytecode generation step.
- More convenient library API.
- Better error messages.

[^1]: [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)
