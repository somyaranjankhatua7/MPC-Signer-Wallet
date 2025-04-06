use std::sync::Arc;

use crate::{routes::handler::auth_handler, services::database::Database};
use axum::{
    Extension, Json, Router,
    routing::{get, post},
};

use super::handler::auth_handler::{RegisterRequest, UserAuthServices};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/user/register", post(|Extension(db): Extension<Arc<Database>>, Json(payload):Json<RegisterRequest>| async move {
            db.register_user(payload).await
        }))
        .route("/user/login", get({ async || "user logged in" }))
}
