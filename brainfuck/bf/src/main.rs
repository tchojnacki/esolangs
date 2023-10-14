use std::process::ExitCode;

use args::Arguments;
use brainlib::{interpreter::Engine, Program, Settings};
use clap::Parser;
use debugger::run_debugger;
use errors::{show_error, CliError};

mod args;
mod debugger;
mod errors;
mod input;
mod source;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            show_error(&err);
            ExitCode::FAILURE
        },
    }
}

fn run() -> Result<(), String> {
    let args = Arguments::parse();
    let settings = Settings::from(&args);
    let debug = settings.debug();
    let source = args.input.get_source()?;
    let program = Program::compile(&source, &settings).map_err(|e| e.message(&source))?;
    let mut eng = Engine::new_std(program, settings);
    match debug {
        true => run_debugger(eng, &source),
        false => eng.run().map_err(|e| e.message(&source)),
    }
}
