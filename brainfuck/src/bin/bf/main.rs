use args::Arguments;
use brainfuck::{compile, VirtualMachine};
use clap::Parser;
use colored::Colorize;
use errors::BfError;
use std::{
    io::{stdin, stdout},
    process::ExitCode,
};

mod args;
mod errors;
mod input;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err.red());
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = Arguments::parse();
    let source = args.input.get_source()?;
    let program = compile(&source, true).map_err(|e| e.message(&source))?;
    let mut vm = VirtualMachine::new(program, 30_000, stdin(), stdout());
    vm.run_all().map_err(|e| e.message(&source))
}
