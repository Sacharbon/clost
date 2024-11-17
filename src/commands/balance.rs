use alloy::{
    primitives::{utils::format_ether, Address},
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::Args;
use eyre::Result;

#[derive(Args)]
pub struct BalanceArgs {
    /// The RPC endpoint
    #[arg(short, long)]
    rpc_url: Option<Url>,

    /// The account to query
    who: Address,
}

#[allow(non_snake_case)]
pub async fn eth_getBalance(args: &BalanceArgs) -> Result<()> {
    let rpc_url = match args.rpc_url {
        Some(ref url) => url.clone(),
        None => "http://localhost/8545".parse()?,
    };
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let weiBalance = provider.get_balance(args.who).await?;
    let ethBalance = format_ether(weiBalance);
    println!("{}", ethBalance);
    Ok(())
}
