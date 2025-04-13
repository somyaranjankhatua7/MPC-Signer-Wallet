use async_trait::async_trait;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{BlockId, BlockNumber, Eip1559TransactionRequest, TransactionReceipt, U256};
use ethers::{
    core::types::Address,
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer},
};
use eyre::Result;
use hex;
use serde::{Deserialize, Serialize};

use crate::routes::handler::response_handler::ErrorResponse;
use crate::{
    chains::features::ChainFeatures, models::user_wallet_model::ChainInfo,
    routes::handler::transaction_handler::Transaction,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ethereum;

impl Ethereum {
    pub async fn send_native(
        chain_data: &ChainInfo,
        payload: &Transaction,
        private_key: String,
    ) -> Result<(ethers::types::Transaction, TransactionReceipt), eyre::Report> {
        let provider = Provider::<Http>::try_from(&chain_data.rpc_url)?;
        let priority_fee = U256::from(2_000_000_000u64);
        let latest_block = provider
            .get_block(BlockId::Number(BlockNumber::Latest))
            .await?
            .expect("latest block error");
        let base_fee = latest_block
            .base_fee_per_gas
            .ok_or_else(|| eyre::eyre!("Missing base fee in latest block"))?;
        let max_fee = base_fee * 2 + priority_fee;

        let from = payload.from.parse::<Address>()?;
        let to = payload.to.parse::<Address>()?;
        let nonce = provider.get_transaction_count(from, None).await?;
        let chain_id = provider.get_chainid().await?;
        let gas = match provider
            .estimate_gas(
                &ethers::types::transaction::eip2718::TypedTransaction::Eip1559(
                    Eip1559TransactionRequest::new()
                        .from(from)
                        .to(to)
                        .value(payload.amount)
                        .nonce(nonce),
                ),
                None,
            )
            .await
        {
            Ok(data) => data,
            Err(e) => {
                return Err(eyre::eyre!("Gas estimation failed: {}", e));
            }
        };

        let tx = Eip1559TransactionRequest::new()
            .from(from)
            .to(to)
            .value(payload.amount)
            .nonce(nonce)
            .gas(gas)
            .max_fee_per_gas(max_fee)
            .max_priority_fee_per_gas(priority_fee);

        let wallet: LocalWallet = private_key.parse()?;
        let client = SignerMiddleware::new(provider, wallet);
        let pending_tx = client.send_transaction(tx, None).await?;

        let receipt = pending_tx
            .await
            .unwrap()
            .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;

        let tx = client
            .get_transaction(receipt.transaction_hash)
            .await?
            .ok_or_else(|| eyre::eyre!("Transaction not found"))?;
        Ok((tx, receipt))
    }
}
