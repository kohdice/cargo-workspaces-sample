use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Hello,
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::Hello => {
                util::greet("bar");
                Ok(())
            }
        }
    }
}
