use std::{fs, io::stdin, path::PathBuf};

use brainlib::util::read_byte;
use clap::Parser;

#[derive(Parser)]
#[command(next_help_heading = "Input")]
#[group(required = true)]
pub struct Input {
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
    pub fn get_source(self) -> Result<String, String> {
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
                    match read_byte(&mut input) {
                        Some(0) | Some(b'!') => break,
                        Some(byte) => output.push(byte as char),
                        None =>
                            return Err("InterpreterError: Unexpected error while reading stdin."
                                .to_owned()),
                    }
                }
                Ok(output)
            },
            _ => unreachable!(),
        }
    }
}
