pub mod balance;
pub mod call;
pub mod send;

use send::SendArgs;
use balance::BalanceArgs;
use call::CallArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    ///Get the balance of an account in wei [aliases: b]
    Balance(BalanceArgs),

    /// Call
    Call(CallArgs),

    ///send
    Send(SendArgs),
}
