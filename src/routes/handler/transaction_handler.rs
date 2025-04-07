use async_trait::async_trait;
use axum::http::StatusCode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, BlockId, BlockNumber, Eip1559TransactionRequest, U256},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Add};

use crate::{
    routes::handler::response_handler::{AxumApiResponse, JsonApiResponse},
    services::database::Database,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub chain_id: String,
    pub tx_type: String,
    pub to: String,
    pub from: String,
    pub amount: u32,
}

#[async_trait]
pub trait UserTransactionServices<T>
where
    T: Serialize + Debug,
{
    async fn create_transaction(&self, payload: Transaction) -> AxumApiResponse<T>;
}

#[async_trait]
impl UserTransactionServices<Eip1559TransactionRequest> for Database {
    async fn create_transaction(
        &self,
        payload: Transaction,
    ) -> AxumApiResponse<Eip1559TransactionRequest> {
        let provider = match Provider::<Http>::try_from("https://eth.llamarpc.com") {
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
                data: Some(tx),
                message: None,
                error: None,
            },
        )
    }
}
