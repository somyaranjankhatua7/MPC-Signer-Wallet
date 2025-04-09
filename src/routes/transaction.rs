use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::database::Database;

use super::handler::transaction_handler::{Transaction, UserTransactionServices};

pub fn transaction_routes() -> Router {
    Router::new().route(
        "/user/native/transfer",
        post(
            |Extension(db): Extension<Arc<Database>>, Json(payload): Json<Transaction>| async move {

        })
    )
}
