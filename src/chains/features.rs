use async_trait::async_trait;
use axum::response::Result;

use crate::{
    models::user_wallet_model::ChainInfo, routes::handler::{response_handler::ErrorResponse, transaction_handler::Transaction},
};

#[async_trait]
pub trait ChainFeatures {
    async fn send_native(
        chain_data: &ChainInfo,
        payload: &Transaction,
        private_key: String,
    ) -> Result<String, ErrorResponse>;
}
