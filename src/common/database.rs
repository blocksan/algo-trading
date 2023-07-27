use futures::TryFutureExt;
// db.rs
use mongodb::{Client, Collection, Database};
use std::sync::Mutex;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use tokio::task;

pub async fn get_database() -> Database {
    task::spawn_blocking(||{
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = Client::with_uri_str("mongodb://localhost:27017")
                .await
                .expect("Failed to connect to MongoDB.");
            client.database("algo_trading")
        })
    })
    .await
    .expect("Failed to spawn blocking task.")
}

