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
use bcrypt::{DEFAULT_COST, hash};
use hex;
use mongodb::{bson::doc, results};
use num_bigint::BigInt;
use rand::{RngCore, TryRngCore, rngs::OsRng};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use sss_rs::basic_sharing;
use std::sync::Arc;
use wcookie::SetCookie;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonApiResponse {
    pub data: Option<UserWalletSchema>,
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

        // Check if user already exists
        if self
            .user_wallet
            .find_one(doc! {"email": &payload.email })
            .await
            .ok()
            .flatten()
            .is_some()
        {
            return AxumApiResponse::Error(
                StatusCode::CONFLICT,
                JsonApiResponse {
                    data: None,
                    message: Some(String::from("User already exists!")),
                    error: None,
                },
            );
        }

        // Generate secret key parts
        let secret_key = KeyServices::generate_secret_key().unwrap();
        let hex_secret_key = hex::encode(secret_key.secret_bytes());
        let part_size = hex_secret_key.len() / 3;
        let (part_one, part_two, part_thr) = (
            &hex_secret_key[..part_size],
            &hex_secret_key[part_size..part_size * 2],
            &hex_secret_key[2 * part_size..],
        );

        // Hash password
        let hash_password = match hash(payload.password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => {
                return AxumApiResponse::Error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonApiResponse {
                        data: None,
                        message: Some(String::from("Unable to hash password")),
                        error: None,
                    },
                );
            }
        };

        // Create cookie
        let mut cookie = SetCookie::new("user_email", &payload.email);
        cookie.http_only = true;
        cookie.path = Some(String::from("/"));

        // Create user schema
        let user_wallet = UserWalletSchema {
            id: None,
            email: payload.email.clone(),
            password: hash_password,
            private_key_a: part_one.to_string(),
            private_key_b: part_two.to_string(),
            private_key_c: part_thr.to_string(),
        };

        // Insert user into database
        let insert_result = match self.user_wallet.insert_one(user_wallet).await {
            Ok(data) => data,
            Err(e) => {
                let error_msg = if e.to_string().contains("E11000 duplicate key error") {
                    "Device ID already exists"
                } else {
                    "Server error"
                };

                return AxumApiResponse::Error(
                    StatusCode::CONFLICT,
                    JsonApiResponse {
                        data: None,
                        message: None,
                        error: Some(error_msg.to_string()),
                    },
                );
            }
        };

        // Fetch inserted user data
        let user_data = match self
            .user_wallet
            .find_one(doc! {"_id": insert_result.inserted_id })
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return AxumApiResponse::Error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonApiResponse {
                        data: None,
                        message: Some(String::from("User registered but data not found")),
                        error: None,
                    },
                );
            }
            Err(_) => {
                return AxumApiResponse::Error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonApiResponse {
                        data: None,
                        message: Some(String::from("Database error while fetching user data")),
                        error: None,
                    },
                );
            }
        };

        // Return success response
        AxumApiResponse::Success(
            StatusCode::OK,
            JsonApiResponse {
                data: Some(user_data),
                message: Some(String::from("User registered successfully")),
                error: None,
            },
        )
    }

    async fn login_user(&self) -> AxumApiResponse {
        unimplemented!()
    }
}
