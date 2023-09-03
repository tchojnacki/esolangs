use args::Arguments;
use brainfuck::{compile, ParseError, RuntimeError, VirtualMachine};
use clap::Parser;
use colored::Colorize;
use std::{
    io::{stdin, stdout},
    process::ExitCode,
};

mod args;
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
    let program = compile(&source, true).map_err(|err| match err {
        ParseError::UnexpectedLoopEnd => "ParseError: Unexpected loop end (found ']').",
        ParseError::MissingLoopEnd => "ParseError: Missing loop end (found EOF).",
    })?;
    let mut vm = VirtualMachine::new(program, 30_000, stdin(), stdout());
    vm.run_all().map_err(|err| {
        match err {
            RuntimeError::InputError => "RuntimeError: Could not read from input.",
            RuntimeError::OutputError => "RuntimeError: Could not write to output.",
        }
        .to_owned()
    })
}
