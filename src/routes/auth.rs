use crate::routes::handler::auth_handler;
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/user/register", post(auth_handler::handle_register))
        .route("/user/login", get({ async || "user logged in" }))
}
