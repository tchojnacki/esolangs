use clap::Parser;

use super::{conventions::Conventions, input::Input};

#[derive(Parser)]
pub struct Arguments {
    #[command(flatten)]
    pub input: Input,

    #[command(flatten)]
    pub conventions: Conventions,
}
