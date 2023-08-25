pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;

use std::{env, time::Instant};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::Serialize;
use serde::Deserialize;
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

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use order_manager::{
    order_dispatcher,
    pnl_state::{self, CurrentPnLState, PnLConfiguration},
    trade_signal_keeper::{self, TradeSignal},
};

use crate::data_consumer::current_market_state::CurrentMarketState;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[post("/fetch_current_pnl")]
async fn fetch_current_pnl() -> impl Responder {
    let current_cache_key = "CPnL_2022_10_18_64d8febebe3ea57f392c36df";
    let only_redis = true;
   let current_pnl_state_option = CurrentPnLState::fetch_current_pnl_state(current_cache_key, only_redis);
   
   if current_pnl_state_option.is_some(){
        let current_pnl_state = current_pnl_state_option.unwrap();
       HttpResponse::Ok().json(current_pnl_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}

#[derive(Deserialize, Debug)]
struct CurrentMarketStateBodyParams{
    current_market_cache_key: String,
}

#[post("/fetch_current_market_state")]
async fn fetch_current_market_state(app_state: web::Data<AppState>, body: web::Json<CurrentMarketStateBodyParams>) -> impl Responder {
    println!("body: {:?}", body);
    let current_cache_key = body.current_market_cache_key.clone();
    let current_market_state_collection = &app_state.current_market_state_collection;
   let current_market_state_option = CurrentMarketState::api_fetch_previous_market_state(current_cache_key.as_str(), current_market_state_collection).await;
   
   if current_market_state_option.is_some(){
        let current_market_state = current_market_state_option.unwrap();
       HttpResponse::Ok().json(current_market_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}

#[derive(Deserialize, Debug)]
struct FetchOrdersBodyParams{
    user_id: String,
}

#[post("/fetch_orders")]
async fn fetch_orders(app_state: web::Data<AppState>, body: web::Json<FetchOrdersBodyParams>) -> impl Responder {
    println!("body: {:?}", body);
    let current_cache_key = body.user_id.clone();
    let current_market_state_collection = &app_state.current_market_state_collection;
   let current_market_state_option = CurrentMarketState::api_fetch_previous_market_state(current_cache_key.as_str(), current_market_state_collection).await;
   
   if current_market_state_option.is_some(){
        let current_market_state = current_market_state_option.unwrap();
       HttpResponse::Ok().json(current_market_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}


struct AppState{
    current_market_state_collection: Collection<CurrentMarketState>,
}

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
    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";

    let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(database_name);

    let current_market_state_collection_name = "current_market_states";
    let current_market_state_collection =db.collection::<CurrentMarketState>(current_market_state_collection_name);


    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState{
            current_market_state_collection: current_market_state_collection.clone(),
        }))
        .service(hello)
        .service(fetch_current_pnl)
        .service(fetch_current_market_state)
        })
        .bind("127.0.0.1:8090")?
        .run()
        .await
}