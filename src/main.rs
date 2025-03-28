#![allow(unused)]

use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api/v1", routes::auth::auth_routes());
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
