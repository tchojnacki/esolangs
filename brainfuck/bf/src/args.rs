use brainlib::Settings;
use clap::{value_parser, Parser, ValueEnum};

use super::input::Input;

#[derive(Parser)]
pub(crate) struct Arguments {
    #[arg(short, long, default_value = "run")]
    pub(crate) target: Target,

    #[command(flatten)]
    pub(crate) input: Input,

    #[command(flatten)]
    conventions: Conventions,
}

impl From<&Arguments> for Settings {
    fn from(args: &Arguments) -> Self {
        Self::try_new(
            args.conventions.tape_length,
            args.conventions.strict,
            args.target == Target::Debug,
        )
        .unwrap()
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    /// Run the code directly from the command line
    Run,

    /// Run the code in debug mode (use # to set a breakpoint)
    Debug,

    /// Compile the code to plain WASM text format
    WasmText,

    /// Compile the code to WASM text format, using WASI
    WasmWasiText,
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
