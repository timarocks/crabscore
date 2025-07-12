//! CLI argument parser for CrabScore

use clap::{Parser, Subcommand};

/// CrabScore – The Rust Efficiency Standard CLI
#[derive(Parser, Debug)]
#[command(
    name = "crabscore",
    version,
    about = "Calculate and inspect CrabScores for Rust projects"
)]
pub struct Cli {
    /// Activate verbose logging (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Calculate a CrabScore for the current project
    Score {
        /// Path to the project or binary to score.
        #[clap(value_parser, default_value = ".")]
        path: String,
        /// Specific binary name or path to benchmark (for Cargo projects).
        #[clap(
            long,
            value_name = "BIN",
            help = "Name (Cargo bin target) or path of executable to benchmark"
        )]
        bin: Option<String>,
    },
    /// Print the current version information
    /// Generate / serve reports
    Report {
        /// Serve a web dashboard instead of just generating files
        #[arg(long)]
        serve: bool,
        /// Port to bind the dashboard to
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    /// Print the current version information
    Version,
}
