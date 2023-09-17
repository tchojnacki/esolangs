use std::{
    io::{stdin, stdout},
    process::ExitCode,
};

use args::Arguments;
use brainfuck::{compile, VirtualMachine};
use clap::Parser;
use colored::Colorize;
use errors::CliError;

mod args;
mod errors;
mod input;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err.red());
            ExitCode::FAILURE
        },
    }
}

fn run() -> Result<(), String> {
    let args = Arguments::parse();
    let settings = (&args).into();
    let source = args.input.get_source()?;
    let program = compile(&source, &settings).map_err(|e| e.message(&source))?;
    let mut vm = VirtualMachine::new(program, settings, stdin(), stdout());
    vm.run().map_err(|e| e.message(&source))
}
