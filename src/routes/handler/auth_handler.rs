use crate::{
    models::user_wallet_model::UserWalletSchema,
    services::{database::Database, key_services},
};
use async_trait::async_trait;
use axum::{
    Extension, Json, body,
    extract::State,
    http::{StatusCode, status},
    response::IntoResponse,
};
use hex;
use mongodb::results;
use num_bigint::BigInt;
use rand::{RngCore, TryRngCore, rngs::OsRng};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use sss_rs::basic_sharing;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub device_id: String,
    pub backup_key: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonApiResponse {
    pub message: Option<String>,
    pub error: Option<String>,
}
pub enum AxumApiResponse {
    Success(StatusCode, JsonApiResponse),
    Error(StatusCode, JsonApiResponse),
}
impl IntoResponse for AxumApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(status, response) => (status, Json(response)).into_response(),
            Self::Error(status, error) => (status, Json(error)).into_response(),
        }
    }
}
#[async_trait]
pub trait UserServices {
    async fn register_user(&self, payload: RegisterRequest) -> AxumApiResponse;
    async fn login_user(&self) -> AxumApiResponse;
}
#[async_trait]
impl UserServices for Database {
    async fn register_user(&self, payload: RegisterRequest) -> AxumApiResponse {
        use crate::services::key_services::KeyServices;
        let secret_key = KeyServices::generate_secret_key().unwrap();
        let hex_secret_key = hex::encode(secret_key.secret_bytes());
        let part_size = hex_secret_key.len() / 3;
        let (part_one, part_two, part_thr) = (
            &hex_secret_key[..part_size],
            &hex_secret_key[part_size..part_size * 2],
            &hex_secret_key[2 * part_size..],
        );

        let user_wallet = UserWalletSchema {
            id: None,
            device_id: payload.device_id,
            backup_key: payload.backup_key,
            private_key_a: part_one.to_string(),
            private_key_b: part_two.to_string(),
            private_key_c: part_thr.to_string(),
        };

        match self.user_wallet.insert_one(user_wallet).await {
            Ok(_) => AxumApiResponse::Success(
                StatusCode::OK,
                JsonApiResponse {
                    message: Some(String::from("User registered Successfully")),
                    error: None,
                },
            ),

            Err(e) => {
                if e.to_string().contains("E11000 duplicate key error") {
                    AxumApiResponse::Error(
                        StatusCode::CONFLICT,
                        JsonApiResponse {
                            message: None,
                            error: Some(String::from("Device ID already exists")),
                        },
                    )
                } else {
                    AxumApiResponse::Error(
                        StatusCode::CONFLICT,
                        JsonApiResponse {
                            message: None,
                            error: Some(String::from("Server error")),
                        },
                    )
                }
            }
        }
    }

    async fn login_user(&self) -> AxumApiResponse {
        unimplemented!()
    }
}
