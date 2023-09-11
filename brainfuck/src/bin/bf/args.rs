use super::{conventions::Conventions, input::Input};
use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    #[command(flatten)]
    pub input: Input,

    #[command(flatten)]
    pub conventions: Conventions,
}
