use serde::{Deserialize, Serialize};
use axum::{Json};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub device_id: String,
    pub backup_key: Option<String>
}   

pub async fn handle_register(Json(payload): Json<RegisterRequest>) -> String {
    println!("{:?}", payload);
    return String::from("USER Register!");
} 