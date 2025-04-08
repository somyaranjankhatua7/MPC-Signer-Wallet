use std::fmt::Debug;

use async_trait::async_trait;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{BlockId, BlockNumber, Eip1559TransactionRequest, U256};
use ethers::{
    core::types::Address,
    signers::{LocalWallet, Signer},
};
use hex;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;

use crate::routes::handler::response_handler::{AxumApiResponse, JsonApiResponse};
use crate::{
    models::user_wallet_model::ChainInfo, routes::handler::transaction_handler::Transaction,
};

pub fn generate_chain_data(secret_key: &str) -> (String, String) {
    let wallet: LocalWallet = secret_key.parse().unwrap();
    let address = wallet.address();
    let address_str = format!("{:#x}", address);
    let uncompressed_public_key = wallet.signer().verifying_key().to_encoded_point(false);
    let public_key = hex::encode(uncompressed_public_key.as_bytes());
    (address_str, public_key)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChainTypeTxn {
    Evm(Eip1559TransactionRequest),
}

pub struct Evm;

#[async_trait]
pub trait UserTxOperations<T>
where
    T: Serialize + Debug,
{
    async fn create_transaction(
        chain_data: &ChainInfo,
        payload: &Transaction,
    ) -> AxumApiResponse<T>;
}

#[async_trait]
impl UserTxOperations<ChainTypeTxn> for Evm {
    async fn create_transaction(
        chain_data: &ChainInfo,
        payload: &Transaction,
    ) -> AxumApiResponse<ChainTypeTxn> {
        let provider = match Provider::<Http>::try_from(&chain_data.rpc_url) {
            Ok(p) => p,
            Err(err) => {
                println!("Error connecting to Ethereum provider: {}", err);
                return AxumApiResponse::SUCCESS(
                    StatusCode::OK,
                    JsonApiResponse {
                        data: None,
                        message: Some(
                            "Unable to connect to Ethereum provider right now.".to_string(),
                        ),
                        error: Some("provider_error".to_string()),
                    },
                );
            }
        };
        let priority_fee = U256::from(2_000_000_000u64);
        let latest_block = provider
            .get_block(BlockId::Number(BlockNumber::Latest))
            .await
            .unwrap()
            .expect("latest block error");
        let base_fee = latest_block.base_fee_per_gas.unwrap();
        let max_fee = base_fee * 2 + priority_fee;

        let from = payload.from.parse::<Address>().unwrap();
        let to = payload.to.parse::<Address>().unwrap();
        let nonce = provider.get_transaction_count(from, None).await.unwrap();
        let chain_id = provider.get_chainid().await.unwrap();

        let gas = provider
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
            .unwrap();

        let tx = Eip1559TransactionRequest::new()
            .from(from)
            .to(to)
            .value(payload.amount)
            .nonce(nonce)
            .gas(gas)
            .max_fee_per_gas(max_fee)
            .max_priority_fee_per_gas(priority_fee);

        AxumApiResponse::SUCCESS(
            StatusCode::OK,
            JsonApiResponse {
                data: Some(ChainTypeTxn::Evm(tx)),
                message: None,
                error: None,
            },
        )
    }
}
