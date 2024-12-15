use alloy::{
    network::{TransactionBuilder, EthereumWallet},
    primitives::{keccak256, Address, Bytes, U256},
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
    /// Params for the function called
    params: String,
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

    let mut input = function_selector.to_vec();

    let params_raw = match args.params.parse::<i32>() {
        Ok(value) => value,
        Err(_) => return Err("Failed to parse params") 
    };

    let number = U256::from(params_raw);
    let number_bytes = number.to_be_bytes_vec();
    input.extend_from_slice(&number_bytes);

    let tx = TransactionRequest::default()
        .with_to(args.to)
        .with_input(Bytes::from(input));

    let result = provider.send_transaction(tx).await?;
    println!("{}", result.tx_hash());
    Ok(())
}

