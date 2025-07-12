use anyhow::Result;
use clap::Parser;
use crabscore_cli::{cli::Cli, command};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    command::execute(cli.command, cli.verbose).await
}
