use std::num::NonZeroUsize;

use brainfuck::{Instruction, VirtualMachineStd};
use colored::Colorize;
use indoc::indoc;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{errors::CliError, source::highlight_source};

enum ReplAction {
    Resume,
    Quit,
    EndSilently,
}

pub fn run_debugger(mut vm: VirtualMachineStd, source: &str) -> Result<(), String> {
    macro_rules! enter_repl {
        () => {
            match repl(&mut vm, source) {
                Ok(ReplAction::Resume) => show("Resuming execution..."),
                Ok(ReplAction::Quit) => {
                    show("Aborting due to a quit from REPL.");
                    return Ok(());
                },
                Ok(ReplAction::EndSilently) => return Ok(()),
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

fn repl(vm: &mut VirtualMachineStd, source: &str) -> Result<ReplAction, ReadlineError> {
    show("Use :r to resume, use :h to see all commands.");
    let mut rl = DefaultEditor::new()?;
    loop {
        let command = match rl.readline("> ") {
            Ok(line) => line,
            Err(ReadlineError::Eof) => return Ok(ReplAction::Resume),
            Err(ReadlineError::Interrupted) => return Ok(ReplAction::Quit),
            Err(other) => return Err(other),
        };

        rl.add_history_entry(&command)?;
        let parts = command.split_whitespace().collect::<Vec<_>>();
        match *parts.first().unwrap_or(&"") {
            ":h" | ":help" => exec_help(),
            ":r" | ":resume" => return Ok(ReplAction::Resume),
            ":s" | ":step" =>
                if let Some(action) =
                    exec_step(vm, source, parts.get(1).copied().unwrap_or("").parse().ok())
                {
                    return Ok(action);
                },
            ":q" | ":quit" => return Ok(ReplAction::Quit),
            _ => show("Invalid command! Use :h to see all commands."),
        }
    }
}

fn exec_help() {
    show(indoc! {"
        Available commands:
          :h, :help       Display the list of available commands
          :r, :resume     Resume the execution
          :s, :step <N>   Execute up to N next instructions [default: 1]
          :q, :quit       Abort the execution
    "});
}

fn exec_step(
    vm: &mut VirtualMachineStd,
    source: &str,
    n: Option<NonZeroUsize>,
) -> Option<ReplAction> {
    let Some(n) = n else {
        show("Invalid number of steps!");
        return None;
    };

    show(format!("Executing the next {n} instructions...").as_str());
    for _ in 0..n.into() {
        match vm.step() {
            Some(Ok(_)) => (),
            Some(Err(err)) => {
                eprintln!("{}", err.message(source).red());
                break;
            },
            None => return Some(ReplAction::EndSilently),
        }
    }

    None
}

fn show(message: impl Colorize) {
    eprintln!("{}", message.yellow());
}
