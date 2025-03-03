use anyhow::Result;
use clap::Parser;

use crate::command::Commands;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}
