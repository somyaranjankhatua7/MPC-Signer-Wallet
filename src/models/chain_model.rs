use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletChainDataSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chain_id: String,
    pub endpoints: Vec<String>,
    pub chain_data: ChainData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainData {
    pub symbol: String,
    pub network: String,
    pub market_cap: String,
    pub total_supply: String,
    pub circulating_supply: String,
    pub perfomance_data: PerfomanceData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerfomanceData {
    pub volume: String,
    pub traders: String,
    pub trades: String,
}
