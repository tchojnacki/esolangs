use brainfuck::{generate, parse, tokenize, ParseError, RuntimeError, VirtualMachine};
use clap::Parser;
use colored::Colorize;
use std::{
    fs,
    io::{stdin, stdout},
    path::PathBuf,
    process::ExitCode,
};

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err.red());
            ExitCode::FAILURE
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Source of the Brainfuck program
    filename: PathBuf,
}

fn run() -> Result<(), &'static str> {
    let args = Args::parse();
    let source = fs::read_to_string(args.filename)
        .map_err(|_| "InterpreterError: Could not read file at specified path.")?;
    let tokens = tokenize(source.chars());
    let ast = parse(tokens).map_err(|err| match err {
        ParseError::UnexpectedLoopEnd => "ParseError: Unexpected loop end (found ']').",
        ParseError::MissingLoopEnd => "ParseError: Missing loop end (found EOF).",
    })?;
    let program = generate(&ast);
    let mut vm = VirtualMachine::new(program, 30_000, stdin(), stdout());
    vm.run_all().map_err(|err| match err {
        RuntimeError::InputError => "RuntimeError: Could not read from input.",
        RuntimeError::OutputError => "RuntimeError: Could not write to output.",
    })
}
