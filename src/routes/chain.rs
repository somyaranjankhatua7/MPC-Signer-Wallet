use axum::{Extension, Json, Router, routing::post};
use std::sync::Arc;

use crate::{models::chain_model::WalletChainDataSchema, services::database::Database};

pub fn chain_routes() -> Router {
    Router::new().route(
        "/protocol/config",
        post(
            |Extension(db): Extension<Arc<Database>>,
             Json(payload): Json<WalletChainDataSchema>| async move {},
        ),
    )
}
