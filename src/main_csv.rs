pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;
pub mod config;

use algo_hub::hammer_pattern::{self, HammerCandle};
use common::redis_client::RedisClient;
use data_consumer::data_consumer_via_csv;
use order_manager::{
    order_dispatcher,
    trade_signal_keeper::{self, TradeSignal},
};
use std::sync::{Arc, Mutex};
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE: f32 = {
        let temp = env::var("HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE")
            .unwrap_or_else(|_| String::from("0.0025"));
        temp.parse::<f32>().unwrap()
    };
    pub static ref HAMMER_RED_CANDLES_COUNT_THRESHOLD: i32 = {
        let temp =
            env::var("HAMMER_RED_CANDLES_COUNT_THRESHOLD").unwrap_or_else(|_| String::from("3"));
        temp.parse::<i32>().unwrap()
    };
    pub static ref HAMMER_MAX_DROP_THRESHOLD_VALUE: f32 = {
        let temp =
            env::var("HAMMER_MAX_DROP_THRESHOLD_VALUE").unwrap_or_else(|_| String::from("20.0"));
        temp.parse::<f32>().unwrap()
    };
    pub static ref HAMMER_MAX_DROP_CANDLE_COUNT: usize = {
        let temp = env::var("HAMMER_MAX_DROP_CANDLE_COUNT").unwrap_or_else(|_| String::from("2"));
        temp.parse::<usize>().unwrap()
    };
    pub static ref HAMMER_SL_MARGIN_POINTS: f32 = {
        let temp = env::var("HAMMER_SL_MARGIN_POINTS").unwrap_or_else(|_| String::from("1.0"));
        temp.parse::<f32>().unwrap()
    };
    pub static ref HAMMER_TARGET_MARGIN_MULTIPLIER: f32 = {
        let temp =
            env::var("HAMMER_TARGET_MARGIN_MULTIPLIER").unwrap_or_else(|_| String::from("1.5"));
        temp.parse::<f32>().unwrap()
    };
}

use mongodb::{options::ClientOptions, Client};

use crate::{
    algo_hub::algo_dispatcher,
    common::{
        enums::{AlgoTypes, TimeFrame},
        raw_stock::RawStock,
    },
};
use crate::order_manager::order_dispatcher::Order;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting CSV-based algorithmic trading analysis...");
    
    // MongoDB connection setup
    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(database_name);
    
    // CSV file path
    let file_5min_path = "ACC_with_indicators_.csv";
    println!("Reading data from: {}", file_5min_path);
    
    
    // Read stock data from CSV
    let stock_5min_data = data_consumer_via_csv::read_5_min_data(file_5min_path)?;
    println!("Successfully loaded {} data points", stock_5min_data.len());
    
    // Initialize collections and utilities
    let hammer_candle_collection = db.collection::<HammerCandle>("hammer_candles");
    let trade_signal_collection = db.collection::<TradeSignal>("trade_signals");
    let orders_collection = db.collection::<Order>("orders");
    
    // Initialize trading components
    let mut hammer_ledger = hammer_pattern::HammerPatternUtil::new();
    let mut trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();
    let mut order_manager = order_dispatcher::OrderManager::new();
    let mut shared_order_ledger: Vec<Order> = Vec::new();
    let redis_client = RedisClient::get_instance();
    
    // Process each stock data point
    println!("Processing stock data...");
    for (index, stock) in stock_5min_data.iter().enumerate() {
        // Print progress every 100 records
        if index % 100 == 0 {
            // println!("Processing record {}/{}", index, stock_5min_data.len());
        }
        
        // Process the stock data through the hammer pattern algorithm
        if let Some(trade_signal) = hammer_ledger.calculate_and_add_ledger(
            stock,
            Some("dummy_pnl_configuration_id".to_string())
        ).await {
            // println!("Found hammer pattern at {}: {:?}", stock.date, trade_signal);
            
            // Add the trade signal to the keeper
            trade_keeper.add_trade_signal(
                &trade_signal
            ).await;
            
            // Create an order based on the trade signal
            order_manager.check_and_dispatch_order(
                trade_signal,
                &redis_client,
                &mut shared_order_ledger
            ).await;
            
            // println!("Order dispatched for symbol: {}", stock.symbol);
        }
    }
    
    // Print summary
    println!("\nProcessing complete!");
    println!("Hammer patterns found: {}", hammer_ledger.fetch_hammer_pattern_ledger().len());
    println!("Trade signals generated: {}", trade_keeper.get_trade_signals().len());
    println!("Orders created: {}", shared_order_ledger.len());
    
    Ok(())
}
