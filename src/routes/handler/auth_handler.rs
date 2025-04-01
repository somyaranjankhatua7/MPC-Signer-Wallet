use async_trait::async_trait;
use axum::{
    Extension, Json, body,
    extract::State,
    http::{StatusCode, status},
    response::IntoResponse,
};
use mongodb::results;
use num_bigint::BigInt;
use rand::{RngCore, TryRngCore, rngs::OsRng};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use sss_rs::basic_sharing;
use std::sync::Arc;

use crate::{
    models::user_wallet_model::UserWalletSchema,
    services::{database::Database, key_services},
};

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
}

#[async_trait]
impl UserServices for Database {
    async fn register_user(&self, payload: RegisterRequest) -> AxumApiResponse {
        use crate::services::key_services::KeyServices;

        let secret_key = KeyServices::generate_secret_key().unwrap();
        let mut split_secret_key =
            KeyServices::split_secret_key(&secret_key.secret_bytes()).unwrap();
        // let extracted_part = split_secret_key.pop().unwrap_or_default();

        let (part_one, part_two, part_three) = match split_secret_key.as_slice() {
            [first, second, third] => (first, second, third),
            _ => panic!("Expected exactly 3 vectors"),
        };

        // let user_wallet = UserWalletSchema {
        //     id: None,
        //     device_id: payload.device_id,
        //     backup_key: payload.backup_key,
        //     private_key_part_one: part_one,
        //     private_key_part_two: part_two,
        //     private_key_part_three: part_three,
        //     user_ipsh_hash: String::from("IPSH"),
        // };

        // match self.user_wallet.insert_one(user_wallet).await {
        //     Ok(_) => AxumApiResponse::Success(
        //         StatusCode::OK,
        //         JsonApiResponse {
        //             message: Some(String::from("User registered Successfully")),
        //             error: None,
        //         },
        //     ),

        //     Err(e) => {
        //         if e.to_string().contains("E11000 duplicate key error") {
        //             AxumApiResponse::Error(
        //                 StatusCode::CONFLICT,
        //                 JsonApiResponse {
        //                     message: None,
        //                     error: Some(String::from("Device ID already exists")),
        //                 },
        //             )
        //         } else {
        //             AxumApiResponse::Error(
        //                 StatusCode::CONFLICT,
        //                 JsonApiResponse {
        //                     message: None,
        //                     error: Some(String::from("Server error")),
        //                 },
        //             )
        //         }
        //     }
        // }
    }
}
