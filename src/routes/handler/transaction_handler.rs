use async_trait::async_trait;
use axum::http::StatusCode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, BlockId, BlockNumber, Eip1559TransactionRequest, U256},
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    ops::Add,
    result::Result::{Err, Ok},
};

use crate::{
    models::user_wallet_model::ChainType,
    routes::handler::response_handler::{AxumApiResponse, JsonApiResponse},
    services::{
        chains_services::{ChainResponse, ChainTypeTxn, TXChain}, database::Database
    },
};

use super::response_handler::{ErrorResponse, SuccessResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub email: String,
    pub chain_id: String,
    pub tx_type: String,
    pub to: String,
    pub from: String,
    pub amount: u32,
}

#[async_trait]
pub trait UserTransactionServices {
    async fn send_native_funds(
        &self,
        payload: Transaction,
    ) -> std::result::Result<SuccessResponse<ChainResponse>, ErrorResponse>;
}

#[async_trait]
impl UserTransactionServices for Database {
    // async fn create_transaction(
    //     &self,
    //     payload: Transaction,
    // ) -> AxumApiResponse<Self::CreateTxResponse> {
    //     let user_data = match self
    //         .user_wallet
    //         .find_one(doc! {"email": &payload.email })
    //         .await
    //     {
    //         Ok(data) => match data {
    //             Some(user) => user,
    //             None => {
    //                 return AxumApiResponse::ERROR(
    //                     StatusCode::NOT_FOUND,
    //                     JsonApiResponse {
    //                         data: None,
    //                         message: None,
    //                         error: Some(String::from("USER_NOT_FOUND!")),
    //                     },
    //                 );
    //             }
    //         },
    //         Err(e) => {
    //             return AxumApiResponse::ERROR(
    //                 StatusCode::FORBIDDEN,
    //                 JsonApiResponse {
    //                     data: None,
    //                     message: None,
    //                     error: Some(String::from("DATABASE_ERROR!")),
    //                 },
    //             );
    //         }
    //     };

    //     let chain_data = match user_data.chains.get(&payload.chain_id) {
    //         Some(data) => data,
    //         None => {
    //             return AxumApiResponse::ERROR(
    //                 StatusCode::NOT_FOUND,
    //                 JsonApiResponse {
    //                     data: None,
    //                     message: None,
    //                     error: Some(String::from("USER_CHAIN_DATA_NOT_FOUND!")),
    //                 },
    //             );
    //         }
    //     };

    //     match chain_data.chain_type {
    //         ChainType::EVM => EVM::create_transaction(&chain_data, &payload).await,
    //     }
    // }

    async fn send_native_funds(
        &self,
        payload: Transaction,
    ) -> std::result::Result<SuccessResponse<ChainResponse>, ErrorResponse> {

        let res = match self
            .user_wallet
            .find_one(doc! {"email": &payload.email})
            .await
        {
            Ok(data) => match data {
                Some(user) => match user.chains.get(&payload.chain_id) {
                    Some(chain_data) => {
                        match chain_data.chain_type {
                            ChainType::EVM => (
                                SuccessResponse {
                                    data: Some(ChainResponse{
                                        chain: TXChain::EVM
                                    }),
                                    message: Some(String::from("OK!")),
                                    status: StatusCode::OK
                                } 
                            )
                        }
                    },
                    None => {
                        return Err(ErrorResponse {
                            error: Some(String::from("USER_CHAIN_DATA_NOT_FOUND!")),
                            status: StatusCode::NOT_FOUND,
                        });
                    }
                },
                None => {
                    return Err(ErrorResponse {
                        error: Some(String::from("USER_NOT_FOUND!")),
                        status: StatusCode::NOT_FOUND,
                    });
                }
            },
            Err(e) => {
                return Err(ErrorResponse {
                    error: Some(String::from("DATABASE_ERROR!")),
                    status: StatusCode::FORBIDDEN,
                });
            }
        };

        Ok(res)

    }
}
