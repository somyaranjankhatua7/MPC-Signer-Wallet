use axum::{
    Extension, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::routes::handler::transaction_handler::{Transaction, UserTransactionServices};
use crate::services::database::Database;

pub fn transaction_routes() -> Router {
    Router::new().route(
        "/user/native/transfer",
        post(
            |Extension(db): Extension<Arc<Database>>, Json(payload): Json<Transaction>| async move {
                match db.send_native_funds(payload).await {
                    Ok(success) => success.into_response(),
                    Err(error) => error.into_response(),
                }
            },
        ),
    )
}
