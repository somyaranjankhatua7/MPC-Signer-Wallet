use crate::models::user_wallet_model::UserWalletSchema;
use dotenv::dotenv;
use mongodb::{Client, Collection, IndexModel, bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use std::env;

pub struct Database {
    pub user_wallet: Collection<UserWalletSchema>,
}

impl Database {
    pub async fn init() -> Self {
        let connection_string = match env::var("MONGO_DATABASE_URL") {
            Ok(value) => value,
            Err(_) => String::from("mongodb://localhost:27017/?directConnection=true"),
        };
        let client = Client::with_uri_str(connection_string)
            .await
            .expect("Failed to connect!");
        let database = client.database("wallet_database");
        let opt = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"device_id": 1})
            .options(opt)
            .build();

        let user_wallet: Collection<UserWalletSchema> = database.collection("user_wallet");
        user_wallet
            .create_index(index)
            .await
            .expect("Failed to create index");

        Database { user_wallet }
    }
}
