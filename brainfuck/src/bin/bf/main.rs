use std::process::ExitCode;

use args::Arguments;
use brainfuck::{compile, Settings, VirtualMachine};
use clap::Parser;
use colored::Colorize;
use debugger::run_debugger;
use errors::CliError;

mod args;
mod debugger;
mod errors;
mod input;
mod source;

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
    let settings = Settings::from(&args);
    let debug = settings.debug();
    let source = args.input.get_source()?;
    let program = compile(&source, &settings).map_err(|e| e.message(&source))?;
    let mut vm = VirtualMachine::new_std(program, settings);
    match debug {
        true => run_debugger(vm, &source),
        false => vm.run().map_err(|e| e.message(&source)),
    }
}
