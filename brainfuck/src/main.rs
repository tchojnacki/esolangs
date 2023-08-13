use std::io::{stdin, stdout};

use brainfuck::{parse, tokenize, VirtualMachine};

fn main() {
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let tokens = tokenize(source.chars());
    let code = parse(tokens).unwrap();
    let mut vm = VirtualMachine::new(stdin(), stdout());
    vm.interpret(&code);
}
