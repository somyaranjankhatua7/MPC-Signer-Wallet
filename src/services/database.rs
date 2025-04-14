use crate::models::{chain_model::WalletChainDataSchema, user_wallet_model::UserWalletSchema};
use dotenv::dotenv;
use mongodb::{Client, Collection, IndexModel, bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use std::env;

pub struct Database {
    pub wallet_chain_data: Collection<WalletChainDataSchema>,
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
            .expect("FAILED TO CONNECT DATABASE!");
        let database = client.database("wallet_database");

        let wallet_chain_data: Collection<WalletChainDataSchema> =
            database.collection("wallet_chain_data");
        wallet_chain_data
            .create_index(Self::create_unique(String::from("chain_id")))
            .await
            .expect("INDEX ERROR: CHAIN_ID DUPLICATE!");

        let user_wallet: Collection<UserWalletSchema> = database.collection("user_wallet");
        user_wallet
            .create_index(Self::create_unique(String::from("email")))
            .await
            .expect("INDEX ERROR: EMAIL DUPLICATE!");

        user_wallet
            .create_index(Self::create_unique(String::from("username")))
            .await
            .expect("INDEX ERROR: USERNAME DUPLICATE!");

        Database {
            user_wallet,
            wallet_chain_data,
        }
    }

    fn create_unique(key: String) -> IndexModel {
        let opt = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! { key: 1})
            .options(opt)
            .build();

        index
    }
}
