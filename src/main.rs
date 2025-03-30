#![allow(unused)]

use axum::{routing::{get, post}, Extension, Router};
use services::database::Database;
use tokio::net::TcpListener;
use std::{env, sync::Arc};
use dotenv::dotenv;
use mongodb::{bson::{Document, doc}, Client, Collection };

mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = Arc::new(Database::init().await);

    let app = Router::new().nest("/api/v1", routes::auth::auth_routes()).layer(Extension(db.clone()));
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
