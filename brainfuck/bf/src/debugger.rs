use std::num::NonZeroUsize;

use brainlib::{interpreter::StdEngine, Instruction};
use colored::Colorize;
use indoc::indoc;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{
    errors::{show_error, CliError},
    source::{highlight_code, highlight_source},
};

enum ReplAction {
    Resume,
    Quit,
    EndSilently,
}

pub(crate) fn run_debugger(mut eng: StdEngine, source: &str) -> Result<(), String> {
    macro_rules! enter_repl {
        () => {
            match repl(&mut eng, source) {
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

    while let Some(result) = eng.step() {
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
            show_error(&err.message(source));
            show("Debugger: Entering debugger due to a runtime error.");
            enter_repl!();
        }
    }

    Ok(())
}

fn repl(eng: &mut StdEngine, source: &str) -> Result<ReplAction, ReadlineError> {
    macro_rules! unwrap_action {
        ($action:expr) => {
            if let Some(action) = $action {
                return Ok(action);
            }
        };
    }

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
        match parts.as_slice() {
            [":c" | ":code"] => exec_code(eng),
            [":h" | ":help"] => exec_help(),
            [":m" | ":memory"] => exec_memory(eng, None),
            [":m" | ":memory", c] => exec_memory(eng, Some(c)),
            [":r" | ":resume"] => return Ok(ReplAction::Resume),
            [":s" | ":step"] => unwrap_action!(exec_step(eng, source, None)),
            [":s" | ":step", n] => unwrap_action!(exec_step(eng, source, Some(n))),
            [":q" | ":quit"] => return Ok(ReplAction::Quit),
            _ => show("Invalid command or arguments! Use :h to see all commands."),
        }
    }
}

fn exec_code(eng: &StdEngine) {
    let source = eng
        .program()
        .code()
        .iter()
        .map(|i| i.to_string())
        .collect::<String>();

    show(highlight_code(&source, eng.pc(), &format!("PC: {}", eng.pc())).as_str());
}

fn exec_help() {
    show(indoc! {"
        Available commands:
          :c, :code          Display the surrounding instructions
          :m, :memory <C>    Display memory around cell C [default: pointer location]
          :r, :resume        Resume the execution
          :s, :step <N>      Execute up to N next instructions [default: 1]
          :q, :quit          Abort the execution
          :h, :help          Display the list of available commands"});
}

fn exec_memory(eng: &StdEngine, c: Option<&str>) {
    let c = c.unwrap_or(&eng.pointer().to_string()).parse::<u32>().ok();
    let Some(c) = c else {
        show("Invalid cell number!");
        return;
    };
    if c >= eng.settings().tape_length() {
        show("Cell number out of range!");
        return;
    }

    let cell = |offset: i32| -> String {
        let i = (c as i32 + offset).rem_euclid(eng.settings().tape_length() as i32) as usize;
        format!("[{:0>3}]", eng.memory()[i])
    };

    show(
        format!(
            "  {} {} {} {} {}\n                ^ #{}",
            cell(-2),
            cell(-1),
            cell(0),
            cell(1),
            cell(2),
            c,
        )
        .as_str(),
    );
}

fn exec_step(eng: &mut StdEngine, source: &str, n: Option<&str>) -> Option<ReplAction> {
    let Some(n) = n.unwrap_or("1").parse::<NonZeroUsize>().ok() else {
        show("Invalid number of steps!");
        return None;
    };

    show(format!("Executing up to {n} next instructions...").as_str());
    for _ in 0..n.into() {
        match eng.step() {
            Some(Ok(instr)) => show(format!("  {instr}").as_str()),
            Some(Err(err)) => {
                show_error(&err.message(source));
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
