use alloy::{
    network::TransactionBuilder,
    primitives::{keccak256, Address, Bytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use clap::Args;
use eyre::Result;

#[derive(Args)]
pub struct CallArgs {
    /// The destination of the transaction
    to: Address,
    /// The signature of the function to call
    sig: String,
}

pub async fn eth_call(args: &CallArgs) -> Result<()> {
    let provider = ProviderBuilder::new()
        .on_builtin("http://localhost:8545")
        .await?;
    let function_selector = Bytes::from(keccak256(args.sig.as_bytes())[..4].to_vec());

    let tx = TransactionRequest::default()
        .with_to(args.to)
        .with_input(function_selector);
    let result = provider.call(&tx).await?;
    println!("{}", result);
    Ok(())
}
