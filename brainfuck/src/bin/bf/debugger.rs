use brainfuck::{Instruction, VirtualMachineStd};
use colored::Colorize;
use indoc::indoc;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{errors::CliError, source::highlight_source};

pub fn run_debugger(mut vm: VirtualMachineStd, source: &str) -> Result<(), String> {
    macro_rules! enter_repl {
        () => {
            match repl() {
                Ok(ReplAction::Resume) => show("Resuming execution..."),
                Ok(ReplAction::Quit) => {
                    show("Aborting due to a quit from REPL.");
                    return Ok(());
                },
                Err(_) => return Err("DebuggerError: Unexpected error in REPL.".to_owned()),
            }
        };
    }

    while let Some(result) = vm.step() {
        if let Ok(Instruction::Breakpoint(pos)) = result {
            show(
                highlight_source(
                    "Debugger: Entering debugger due to a breakpoint hit.",
                    source,
                    pos as usize,
                    "breakpoint defined here",
                )
                .as_str(),
            );
            enter_repl!();
        } else if let Err(err) = result {
            eprintln!("{}", err.message(source).red());
            show("Debugger: Entering debugger due to a runtime error.");
            enter_repl!();
        }
    }
    Ok(())
}

enum ReplAction {
    Resume,
    Quit,
}

fn repl() -> Result<ReplAction, ReadlineError> {
    show("Use :r to resume, use :h to see all commands.");
    let mut rl = DefaultEditor::new()?;
    loop {
        let command = match rl.readline("> ") {
            Ok(line) => line,
            Err(ReadlineError::Eof) => return Ok(ReplAction::Resume),
            Err(ReadlineError::Interrupted) => return Ok(ReplAction::Quit),
            Err(other) => return Err(other),
        };

        if let Some(command) = command.strip_prefix(':') {
            rl.add_history_entry(format!(":{command}"))?;
            let parts = command.split_whitespace().collect::<Vec<_>>();
            match parts[0] {
                "h" | "help" => print_help(),
                "r" | "resume" => return Ok(ReplAction::Resume),
                "q" | "quit" => return Ok(ReplAction::Quit),
                _ => show("Unknown command! Use :h to see all commands."),
            }
        } else {
            show("Executing provided code...");
        }
    }
}

fn print_help() {
    show(indoc! {"
        Prompts which do not start with ':' are interpreted as code.
        Available commands:
            :h, :help      Display the list of available commands
            :r, :resume    Resume the execution
            :q, :quit      Abort the execution
    "});
}

fn show(message: impl Colorize) {
    eprintln!("{}", message.yellow());
}
