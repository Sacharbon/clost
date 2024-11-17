pub mod balance;

use balance::BalanceArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    ///Get the balance of an account in wei [aliases: b]
    Balance(BalanceArgs),
}
