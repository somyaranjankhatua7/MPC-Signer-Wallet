use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletChainDataSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    chain_id: String,
    endpoints: Vec<String>,
    chain_data: ChainData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainData {
    symbol: String,
    network: String,
    market_cap: String,
    total_supply: String,
    circulating_supply: String,
    perfomance_data: PerfomanceData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerfomanceData {
    volume: String,
    traders: String,
    trades: String,
}
