use axum::{
    Extension, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    models::chain_model::WalletChainDataSchema, routes::handler::chain_handler,
    services::database::Database,
};

pub fn chain_routes() -> Router {
    Router::new()
        .route(
            "/protocol/config",
            post(
                |Extension(db): Extension<Arc<Database>>,
                 Json(payload): Json<WalletChainDataSchema>| async move {
                    match db.config_chain(payload).await {
                        Ok(success) => success.into_response(),
                        Err(error) => error.into_response(),
                    }
                },
            ),
        )
        .route(
            "/get/protocols",
            get(|Extension(db): Extension<Arc<Database>>| async move {
                match db.get_protocols().await {
                    Ok(success) => success.into_response(),
                    Err(error) => error.into_response(),
                }
            }),
        )
}
