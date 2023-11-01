# `bf` 🧠

> [!NOTE]  
> Visit the [brainfuck](../#readme) directory to learn more about the language.

![](./docs/mandelbrot.gif)

## Examples

```
$ bf -c "+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+."
hello, world!


$ bf --help
Usage: bf [OPTIONS] <--file <FILE>|--code <CODE>|--stdin>

Options:
  -t, --target <TARGET>
          [default: run]

          Possible values:
          - run:            Run the code directly from the command line
          - debug:          Run the code in debug mode (use # to set a breakpoint)
          - wasm-text:      Compile the code to plain WASM text format
          - wasm-wasi-text: Compile the code to WASM text format, using WASI

  -h, --help
          Print help (see a summary with '-h')

Input:
  -f, --file <FILE>
          Path to the file containing the program code

  -c, --code <CODE>
          Program code passed as an inline argument

  -s, --stdin
          Pass the program code through stdin and (use ! to separate it from input)

Conventions:
      --length <TAPE_LENGTH>
          Count of available memory cells
          
          [default: 30000]

      --strict
          If enabled, stop execution when overflowing a cell or tape index


$ bf -f specs/add.code.bf -t debug
Debugger: Entering debugger due to a breakpoint hit.
  | es ]␤#␤++␤>
  |      ^ breakpoint defined here
  |        at 2:1
Use :r to resume, use :h to see all commands.
> :h
Available commands:
  :c, :code          Display the surrounding instructions
  :m, :memory <C>    Display memory around cell C [default: pointer location]
  :r, :resume        Resume the execution
  :s, :step <N>      Execute up to N next instructions [default: 1]
  :q, :quit          Abort the execution
  :h, :help          Display the list of available commands
> :s 10
Executing up to 10 next instructions...
  +
  +
  >
  +
  +
  +
  +
  +
  [
  <
> :c
  | +++[<+>-]++
  |      ^ PC: 16
> :m
  [000] [000] [002] [005] [000]
                ^ #0
> :r
Resuming execution...
7


$ bf -c ",[.,]" -t wasm-text
(module
  (import "bf" "input" (func $read_byte (result i32)))
  (import "bf" "output" (func $write_byte (param i32)))
  (memory (;0;) 1 1)
  (global $ptr (mut i32) (i32.const 0))
  (func $main 
    (global.get $ptr)
    (call $read_byte)
    (i32.const 255)
    (i32.and)
    (i32.store8)
    (block 
      (global.get $ptr)
      (i32.load8_u)
      (i32.eqz)
      (br_if 0)
      (loop 
        (global.get $ptr)
        (i32.load8_u)
        (call $write_byte)
        (global.get $ptr)
        (call $read_byte)
        (i32.const 255)
        (i32.and)
        (i32.store8)
        (global.get $ptr)
        (i32.load8_u)
        (br_if 0)
      )
    )
  )
  (export "memory" (memory 0))
  (export "_start" (func $main))
)
```

## Features

- Covered by integration tests.
- Reading code from file, as an argument or from stdin.
- Compilation to WebAssembly.
- Built-in debugger with breakpoints and stepping.
