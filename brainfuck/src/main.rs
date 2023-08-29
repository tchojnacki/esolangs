use brainfuck::{compile, ParseError, RuntimeError, VirtualMachine};
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
    /// Filename of the Brainfuck program
    #[arg(short, long)]
    file: PathBuf,
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let source = fs::read_to_string(&args.file).map_err(|_| {
        format!(
            "InterpreterError: Could not read file at path: {}",
            fs::canonicalize(&args.file).unwrap_or(args.file).display()
        )
    })?;
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
