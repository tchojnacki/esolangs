use std::{io::stdout, process::ExitCode};

use args::{Arguments, Target};
use brainlib::{interpreter::Engine, wasm::WasmModule, Program, Settings};
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
    let source = args.input.get_source()?;
    let program = Program::compile(&source, &settings).map_err(|e| e.message(&source))?;

    match args.target {
        Target::Debug => run_debugger(Engine::new_std(program, settings), &source),
        Target::Run => Engine::new_std(program, settings)
            .run()
            .map_err(|e| e.message(&source)),
        Target::WasmWasiText => WasmModule::compile_from(program)
            .emit_wat(stdout())
            .map_err(|_| "Error: Could not write to stdout.".into()),
    }
}
