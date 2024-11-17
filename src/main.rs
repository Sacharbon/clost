use clap::Parser;
use commands::{balance::eth_getBalance, Commands};

pub mod commands;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Perform Ethereum RPC calls from the comfort of your command line",
    after_help = "Made by love at 3:41 pm"
)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let _ = match &cli.commands {
        Commands::Balance(args) => eth_getBalance(args).await,
    };
}
