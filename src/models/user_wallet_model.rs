use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWalletSchema {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub device_id: String,
    pub backup_key: Option<String>,
    pub private_key_part_one: Vec<(u8, u8)>,
    pub private_key_part_two: Vec<(u8, u8)>,
    pub private_key_part_three: Vec<(u8, u8)>,
    pub user_ipsh_hash: String
}