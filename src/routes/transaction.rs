use axum::{Extension, Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::database::Database;

use super::handler::auth_handler::{TransactionRequest, UserServices};

pub fn transaction_routes() -> Router {
    Router::new().route("/user/create/transaction", get(|Extension(db): Extension<Arc<Database>>, Json(payload): Json<TransactionRequest>| async move {
        db.create_transaction(payload).await
    }))
}
