use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[clap(author, version, long_about = "A util for compress simple text files")]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
