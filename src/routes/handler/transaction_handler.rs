use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::fmt::Debug;

use crate::{routes::handler::response_handler::{AxumApiResponse, JsonApiResponse}, services::database::Database};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    pub chain_id: String,
    pub tx_type: String,
    pub to: String,
    pub from: String,
    pub amount: u32
}

#[async_trait]
pub trait UserTransactionServices<T> 
where 
    T: Serialize + Debug {
    async fn create_transaction(&self, payload: TransactionRequest) -> AxumApiResponse<T>;
}

#[async_trait]
impl UserTransactionServices<TransactionRequest> for Database {
    async fn create_transaction(&self, payload: TransactionRequest) -> AxumApiResponse<TransactionRequest> {
        unimplemented!();
    }
}