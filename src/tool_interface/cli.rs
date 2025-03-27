use clap::Parser;
use crate::tool_interface::commands::Commands;
#[derive(Parser)]
#[command(name = "naturalTerminal", about = "A natural language CLI tool that echoes your input")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
