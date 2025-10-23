mod case_combinations;
mod character_combinations;
mod cli;
mod leet_combinations;
mod pipeline;
mod sanitize;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    pipeline::run(cli)
}
