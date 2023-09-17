use brainfuck::Settings;
use clap::{value_parser, Parser};

use super::input::Input;

#[derive(Parser)]
pub struct Arguments {
    /// Run code in debug mode (use # to set a breakpoint)
    #[arg(short, long)]
    debug: bool,

    #[command(flatten)]
    pub input: Input,

    #[command(flatten)]
    conventions: Conventions,
}

impl From<&Arguments> for Settings {
    fn from(args: &Arguments) -> Self {
        Self::try_new(
            args.conventions.tape_length,
            args.conventions.strict,
            args.debug,
        )
        .unwrap()
    }
}

#[derive(Parser)]
#[command(next_help_heading = "Conventions")]
#[group()]
struct Conventions {
    /// Count of available memory cells
    #[arg(long = "length", default_value_t = Settings::DEFAULT_LENGTH, value_parser = value_parser!(u32).range(3..=1_000_000_000))]
    tape_length: u32,

    /// If enabled, stop execution when overflowing a cell or tape index
    #[arg(long)]
    strict: bool,
}
