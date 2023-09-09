pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;
pub mod config;

extern crate mongodb;
extern crate tokio;
use mongodb::{options::ClientOptions, Client};

use crate::common::raw_stock::RawStockFromFile;
use crate::common::date_parser;
use crate::common::enums::TimeFrame;

use csv::Reader;
use mongodb::{
    bson::{doc, Document},
    error::Result
};

use std::env;
use lazy_static::lazy_static;

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

#[tokio::main]
async fn main() -> Result<()> {
    // MongoDB connection options
    let mongo_uri = "mongodb://localhost:27017";
    let db_name = "algo_trading";

    // Parse the MongoDB connection string
    let client_options = ClientOptions::parse(mongo_uri).await?;
    
    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;

    // let processed_stock_symbols = ["ACC","ADANIENT","ADANIGREEN","ADANIPORTS","AMBUJACEM","APOLLOHOSP","ASIANPAINT","AUROPHARMA","AXISBANK","BAJAJ-AUTO","BAJAJFINSV","BAJAJHLDNG","BAJFINANCE","BANDHANBNK","BANKBARODA","BERGEPAINT","BHARTIARTL"];
    let stock_symbols = ["BIOCON","BOSCHLTD","BPCL","BRITANNIA","CHOLAFIN","CIPLA","COALINDIA","COLPAL","DABUR","DIVISLAB","DLF","DMART","DRREDDY","EICHERMOT","GAIL","GLAND","GODREJCP","GRASIM","HAVELLS","HCLTECH","HDFC","HDFCAMC","HDFCBANK","HDFCLIFE","HEROMOTOCO","HINDALCO","HINDPETRO","HINDUNILVR","ICICIBANK","ICICIGI","ICICIPRULI","IGL","INDIGO","INDUSINDBK","INDUSTOWER","INFY","IOC","ITC","JINDALSTEL","JSWSTEEL","JUBLFOOD","KOTAKBANK","LICI","LT","LTI","LUPIN","MARICO","MARUTI","MCDOWELL-N","MM","MUTHOOTFIN","NAUKRI","NESTLEIND","NIFTY 50","NIFTY BANK","NMDC","NTPC","ONGC","PEL","PGHH","PIDILITIND","PIIND","PNB","POWERGRID","RELIANCE","SAIL","SBICARD","SBILIFE","SBIN","SHREECEM","SIEMENS","SUNPHARMA","TATACONSUM","TATAMOTORS","TATASTEEL","TCS","TECHM","TITAN","TORNTPHARM","ULTRACEMCO","UPL","VEDL","WIPRO","YESBANK"];

    // Access the specific database and collection
    let db = client.database(db_name);

    for stock_symbol in stock_symbols {
        println!("Processing stock symbol: {}", stock_symbol);
        let collection_name = stock_symbol.clone();
        let collection = db.collection::<Document>(collection_name.trim().to_lowercase().as_str());

        // Read the CSV file
        let time_frames = ["minute_data.csv","3minute_data.csv", "5minute_data.csv", "10minute_data.csv", "15minute_data.csv", "30minute_data.csv", "60minute_data.csv", "day_data.csv"];
        
        for time_frame in time_frames {
            let file_path = format!("{}{}_{}","/Users/sandeepghosh/Documents/personal-projects/rust/datasets_all_intervals_NSE/" , stock_symbol, time_frame);
            println!("Reading file: {}", file_path);
            let file = std::fs::File::open(file_path)?;
            let mut rdr = Reader::from_reader(file);
            let mut first_row = false;
            // Iterate over the CSV records and insert them into MongoDB
            for result in rdr.deserialize::<RawStockFromFile>() {
                let record: RawStockFromFile = result.unwrap();
                let date_time = date_parser::parse_date_in_stock_format(record.date.as_str());
                if first_row == false {
                    first_row = true;
                    continue;
                }
                // Create a BSON document from the CSV record
                let doc = doc! {
                    "symbol": stock_symbol.clone(),
                    "date": date_time.unwrap(),
                    "close": record.close,
                    "high": record.high,
                    "low": record.low,
                    "open": record.open,
                    "volume": record.volume,
                    "market_time_frame": match time_frame {
                        "minute_data.csv" => TimeFrame::OneMinute.to_string(),
                        "3minute_data.csv" => TimeFrame::ThreeMinutes.to_string(),
                        "5minute_data.csv" => TimeFrame::FiveMinutes.to_string(),
                        "10minute_data.csv" => TimeFrame::FifteenMinutes.to_string(),
                        "15minute_data.csv" => TimeFrame::FifteenMinutes.to_string(),
                        "30minute_data.csv" => TimeFrame::SixtyMinutes.to_string(),
                        "60minute_data.csv" => TimeFrame::SixtyMinutes.to_string(),
                        "day_data.csv" => TimeFrame::OneDay.to_string(),
                        _ => TimeFrame::OneMinute.to_string()
                    }
                };
                
                // Insert the document into the MongoDB collection
                collection.insert_one(doc, None).await?;
            }
        }
    }


    println!("Data insertion completed successfully.");

    Ok(())
}
