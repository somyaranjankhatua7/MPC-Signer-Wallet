use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainInfo {
    pub index: u32,
    pub public_key: String,
    pub address: String,
    pub balance: String,
    pub rpc_url: String,
    pub chain_type: ChainType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChainType {
    EVM,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWalletSchema {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub private_key_a: String,
    pub private_key_b: String,
    pub private_key_c: String,
    pub chains: HashMap<String, ChainInfo>
}