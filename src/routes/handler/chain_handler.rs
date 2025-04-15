use async_trait::async_trait;
use axum::http::StatusCode;
use futures::TryStreamExt;
use mongodb::bson::doc;
use tracing::instrument::WithSubscriber;

use crate::{
    models::chain_model::WalletChainDataSchema,
    routes::handler::response_handler::{AxumApiResponse, ErrorResponse, SuccessResponse},
    services::database::Database,
};

impl Database {
    pub async fn config_chain(
        &self,
        payload: WalletChainDataSchema,
    ) -> std::result::Result<SuccessResponse<String>, ErrorResponse> {
        if self
            .wallet_chain_data
            .find_one(doc! {"chain_id": &payload.chain_id })
            .await
            .ok()
            .flatten()
            .is_some()
        {
            return Err(ErrorResponse {
                error: Some(String::from("CHAIN EXIST!")),
                status: StatusCode::NOT_FOUND,
            });
        }

        match self.wallet_chain_data.insert_one(payload).await {
            Ok(_) => {
                return Ok(SuccessResponse {
                    data: Some(String::from("DATA")),
                    message: Some(String::from("CHAIN CONFIG SUCCESSFULLY")),
                    status: StatusCode::OK,
                });
            }
            Err(_) => {
                return Err(ErrorResponse {
                    error: Some(String::from("ERROR CONFIG CHAIN")),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                });
            }
        }
    }

    pub async fn get_protocols(
        &self,
    ) -> std::result::Result<SuccessResponse<Vec<WalletChainDataSchema>>, ErrorResponse> {
        let data = match self.wallet_chain_data.find(doc! {}).await {
            Ok(mut data) => data.try_collect().await.unwrap_or_else(|_| vec![]),
            Err(_) => {
                return Err(ErrorResponse {
                    error: Some(String::from("ERROR!")),
                    status: StatusCode::NOT_FOUND,
                });
            }
        };

        Ok(SuccessResponse {
            data: Some(data),
            message: Some(String::from("PROTOCOL DATA!")),
            status: StatusCode::OK,
        })
    }
}
