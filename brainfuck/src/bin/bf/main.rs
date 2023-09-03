use brainfuck::{compile, read_u8, ParseError, RuntimeError, VirtualMachine};
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
#[command(next_help_heading = "Input")]
#[group(required = true)]
struct Input {
    /// Path to the file containing the program code
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Program code passed as an inline argument
    #[arg(short, long)]
    code: Option<String>,

    /// Pass the program code through stdin and (use ! to separate it from input)
    #[arg(short, long)]
    stdin: bool,
}

impl Input {
    fn get_source(self) -> Result<String, String> {
        match (self.file, self.code, self.stdin) {
            (Some(path), None, false) => fs::read_to_string(&path).map_err(|_| {
                format!(
                    "InterpreterError: Could not read file at path: {}",
                    fs::canonicalize(&path).unwrap_or(path).display()
                )
            }),
            (None, Some(code), false) => Ok(code),
            (None, None, true) => {
                let mut input = stdin().lock();
                let mut output = String::new();
                loop {
                    match read_u8(&mut input) {
                        Some(0) | Some(b'!') => break,
                        Some(byte) => output.push(byte as char),
                        None => {
                            return Err("InterpreterError: Unexpected error while reading stdin."
                                .to_owned())
                        }
                    }
                }
                Ok(output)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Parser)]
struct Args {
    #[command(flatten)]
    input: Input,
}

fn run() -> Result<(), String> {
    let args = Args::parse();
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
