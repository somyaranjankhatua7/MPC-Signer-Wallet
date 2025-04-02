use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWalletSchema {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub device_id: String,
    pub backup_key: Option<String>,
    pub private_key_a: String,
    pub private_key_b: String,
    pub private_key_c: String,
}