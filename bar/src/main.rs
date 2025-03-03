mod cli;
mod command;

use anyhow::Result;
use clap::Parser;

use cli::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    args.run()
}
