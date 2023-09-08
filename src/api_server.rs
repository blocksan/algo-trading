pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;
pub mod api;
pub mod config;

use std::env;
use api::routes::routes_config;
use api::utils::app_state::AppState;
use lazy_static::lazy_static;
use config::mongodb_connection;
lazy_static! {
    static ref HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE: f32 = {
        let temp = env::var("HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };

    static ref HAMMER_RED_CANDLES_COUNT_THRESHOLD: i32 = {
        let temp = env::var("HAMMER_RED_CANDLES_COUNT_THRESHOLD").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<i32>().unwrap()
    };

    static ref HAMMER_MAX_DROP_THRESHOLD_VALUE: f32 = {
        let temp = env::var("HAMMER_MAX_DROP_THRESHOLD_VALUE").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };

    static ref HAMMER_MAX_DROP_CANDLE_COUNT: usize = {
        let temp = env::var("HAMMER_MAX_DROP_CANDLE_COUNT").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<usize>().unwrap()
    };

    static ref HAMMER_SL_MARGIN_POINTS: f32 = {
        let temp = env::var("HAMMER_SL_MARGIN_POINTS").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };

    static ref HAMMER_TARGET_MARGIN_MULTIPLIER:f32 = {
        let temp = env::var("HAMMER_TARGET_MARGIN_MULTIPLIER").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };
    
}

use actix_web::{ App, HttpServer, web};

use crate::data_consumer::current_market_state::CurrentMarketState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| String::from("development"));
    match environment.as_str() {
        "production" => {
            dotenv::from_filename(".env").ok();
            println!("Using production environment variables");
        }
        _ => {
            dotenv::from_filename(".env.dev").ok();
            println!("Using development environment variables");
        }
    }
    // let mongo_url = "mongodb://localhost:27017";
    // let database_name = "algo_trading";

    // let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();

    // let db = client.database(database_name);

    let current_market_state_collection_name = "current_market_states";
    let current_market_state_collection =mongodb_connection::fetch_db_connection().await.collection::<CurrentMarketState>(current_market_state_collection_name);


    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState{
            current_market_state_collection: current_market_state_collection.clone(),
        }))
        .configure(|cfg| {
            routes_config(cfg);
        })
        })
        .bind("127.0.0.1:8090")?
        .run()
        .await
}