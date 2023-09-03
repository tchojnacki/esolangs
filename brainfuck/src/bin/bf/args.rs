use super::input::Input;
use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    #[command(flatten)]
    pub input: Input,
}
