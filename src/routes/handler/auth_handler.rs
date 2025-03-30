use std::sync::Arc;

use crate::{models::user_wallet_model::UserWalletSchema, services::database::Database};
use axum::{response::IntoResponse, Extension, Json, http::StatusCode};

use num_bigint::BigInt;
use rand::{RngCore, TryRngCore, rngs::OsRng};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use sss_rs::basic_sharing;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub device_id: String,
    pub backup_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub message: String,
}

fn generate_secret_key() -> SecretKey {
    let mut rng = OsRng;
    let mut random_bytes = [0u8; 32];
    rng.try_fill_bytes(&mut random_bytes);
    SecretKey::from_byte_array(&random_bytes).expect("Failed to generate private key")
}

fn split_secret_key(
    secret_key: &[u8],
) -> std::result::Result<Vec<Vec<(u8, u8)>>, basic_sharing::Error> {
    basic_sharing::from_secrets(secret_key, 3, 6, None)
}

pub async fn handle_register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // let secret_key = generate_secret_key();
    // println!("{:?}", secret_key.secret_bytes());

    

    // let big = split_secret_key(&secret_key.secret_bytes());
    // println!("{:?}", big);

    let user_wallet = UserWalletSchema {
        id: None,
        device_id: payload.device_id,
        backup_key: payload.backup_key
    };
    
    match db.user_wallet.insert_one(user_wallet).await {
        Ok(_) => (StatusCode::OK, Json(RegisterResponse { message: String::from("RegisterResponse")})),
        Err(e) => {
            if e.to_string().contains("E11000 duplicate key error") {
                (StatusCode::CONFLICT,Json(RegisterResponse { message: String::from("Device ID already exists")}))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterResponse { message: String::from("Server error")}))
                
            }
        }
    }
}
