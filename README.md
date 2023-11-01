# esolangs 🎨

Collection of Rust projects based around [esoteric programming languages](https://en.wikipedia.org/wiki/Esoteric_programming_language).

See more details in subfolder READMEs.

Implemented languages:

- [**Brainfuck 🧠 (`brainfuck`)**](./brainfuck#readme)
  - [`brainlib` 📦](./brainfuck/brainlib#readme)
  - [`bf` 🛠️](./brainfuck/bf#readme)

Common building blocks:

- [`wasmitter` 📦](./common/wasmitter#readme) - emits WebAssembly code

> [!NOTE]
> - 🛠️ - binary - this can be run from the command line
> - 📦 - library - this can be imported by a Rust program

> [!WARNING]
> All of the library crates in this repository are subject to breaking changes
> (with or without a version change) if they haven't reached `1.0.0` yet.
