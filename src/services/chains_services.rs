use std::fmt::Debug;
use async_trait::async_trait;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{BlockId, BlockNumber, Eip1559TransactionRequest, TransactionReceipt, U256};
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
    EVM,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub chain: TXChain
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TXChain {
    EVM(EVMResponse)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EVMResponse {
    pub transaction: ethers::types::Transaction,
    pub recepient: TransactionReceipt
}