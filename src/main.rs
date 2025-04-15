#![allow(unused)]

use axum::{
    Extension, Router,
    routing::{get, post},
};
use dotenv::dotenv;
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};
use services::database::Database;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

mod chains;
mod errors;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = Arc::new(Database::init().await);

    let app = Router::new()
        .nest("/api/v1", routes::chain::chain_routes())
        .nest("/api/v1", routes::auth::auth_routes())
        .nest("/api/v1", routes::transaction::transaction_routes())
        .layer(Extension(db.clone()));

    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
