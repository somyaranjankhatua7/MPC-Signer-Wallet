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
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use wcookie::SetCookie;

use crate::{models::user_wallet_model::ChainType, routes::handler::response_handler::{AxumApiResponse, JsonApiResponse}};
use crate::{
    models::user_wallet_model::{UserWalletSchema, ChainInfo},
    services::{database::Database, key_services},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}


#[async_trait]
pub trait UserAuthServices<T> 
where 
    T: Serialize + Debug
{
    async fn register_user(&self, payload: RegisterRequest) -> AxumApiResponse<T>;
    async fn login_user(&self) -> AxumApiResponse<T>;
}

#[async_trait]
impl UserAuthServices<UserWalletSchema> for Database {
    async fn register_user(&self, payload: RegisterRequest) -> AxumApiResponse<UserWalletSchema> {
        use crate::services::key_services::KeyServices;
        use crate::services::chains_services::generate_chain_data;

        // Check if user already exists
        if self
            .user_wallet
            .find_one(doc! {"email": &payload.email })
            .await
            .ok()
            .flatten()
            .is_some()
        {
            return AxumApiResponse::ERROR(
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
                return AxumApiResponse::ERROR(
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

        // Addind default chain configurations
        let (_address, _public_key) = generate_chain_data(&hex_secret_key);
        let mut chains = HashMap::new();
        chains.insert(String::from("1"), ChainInfo {
            index: 1,
            public_key: _public_key,
            address: _address,
            balance: String::from("0"),
            rpc_url: String::from("https://eth.llamarpc.com"),
            chain_type: ChainType::EVM
        });

        // Create user schema
        let user_wallet = UserWalletSchema {
            id: None,
            email: payload.email.clone(),
            password: hash_password,
            private_key_a: part_one.to_string(),
            private_key_b: part_two.to_string(),
            private_key_c: part_thr.to_string(),
            chains
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

                return AxumApiResponse::SUCCESS(
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
                return AxumApiResponse::ERROR(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonApiResponse {
                        data: None,
                        message: Some(String::from("User registered but data not found")),
                        error: None,
                    },
                );
            }
            Err(_) => {
                return AxumApiResponse::ERROR(
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
        AxumApiResponse::SUCCESS(
            StatusCode::OK,
            JsonApiResponse {
                data: Some(user_data),
                message: Some(String::from("User registered successfully")),
                error: None,
            },
        )
    }

    async fn login_user(&self) -> AxumApiResponse<UserWalletSchema> {
        unimplemented!()
    }
    
}
