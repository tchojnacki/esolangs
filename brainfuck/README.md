# Brainfuck

> Brainfuck is an esoteric programming language created in 1993 by Urban Müller. Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers [^1].

## Features

- Fully implemented lexer, parser, bytecode generator and interpreter.
- Can be used as a binary or library.
- Unit and integration test coverage.
- Reading code from file, as an argument or from stdin.
- Implementation choices:
  - Cell size: 8 bit unsigned (with wrapping)
  - Cell count: 30 000 (with wrapping)
  - Starting cell index: 0

## Examples

### Binary

```
bf --help
bf -f specs/mandelbrot.code.bf
bf -c "++>+++++[<+>-]++++++++[<++++++>-]<."
```

### Library

```Rust
let program = compile(
  "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.",
  true
).expect("Parse error!");

VirtualMachine::new_std(program).run_all().unwrap();
```

## TODO

- Customizable wrapping.
- Customizable tape length.
- Built-in [debugger](https://esolangs.org/wiki/Brainfuck#Extensions).
- Built-in code optimizer.

[^1]: [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)
