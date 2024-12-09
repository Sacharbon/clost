use alloy::{
    network::{TransactionBuilder, EthereumWallet},
    primitives::{keccak256, Address, Bytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use clap::Args;
use eyre::Result;

#[derive(Args)]
pub struct SendArgs {
    /// The destination of the transaction,
    to: Address,
    /// The signature of the function to call
    sig: String,
    /// The private key of the account executing the transaction
    private_key: PrivateKeySigner,
}

pub async fn eth_send(args: &SendArgs) -> Result<()> {
    let wallet = EthereumWallet::from(args.private_key.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_builtin("http://localhost:8545")
        .await?;
    let function_selector = Bytes::from(keccak256(args.sig.as_bytes())[..4].to_vec());

    let tx = TransactionRequest::default()
        .with_to(args.to)
        .with_input(function_selector);
    let result = provider.send_transaction(tx).await?;
    println!("foo {}", result.tx_hash());
    Ok(())
}

