use brainfuck::Settings;
use clap::{value_parser, Parser};

#[derive(Parser)]
#[command(next_help_heading = "Conventions")]
#[group()]
pub struct Conventions {
    /// Count of available memory cells
    #[arg(long = "length", default_value_t = Settings::DEFAULT_LENGTH, value_parser = value_parser!(u32).range(3..=1_000_000_000))]
    tape_length: u32,

    /// If enabled, stop execution when overflowing a cell or tape index
    #[arg(long)]
    strict: bool,
}

impl From<Conventions> for Settings {
    fn from(conv: Conventions) -> Self {
        Self::try_new(conv.tape_length, conv.strict).unwrap()
    }
}
