use mongodb::{bson::{doc, Document}, Collection, options::FindOptions};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;

use crate::{common::enums::TimeFrame, config::mongodb_connection};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code, unused_variables)]
pub struct RawStock {
    pub symbol: String,
    pub date: String,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub volume: i32,
    pub market_time_frame: TimeFrame
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code, unused_variables)]
pub struct RawStockFromFile {
    pub date: String,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub volume: i32
}

#[allow(dead_code, unused_variables)]
impl RawStock{
    pub fn new(symbol: String, date: String, close: f32, high: f32, low: f32, open: f32, volume: i32, market_time_frame: TimeFrame) -> RawStock {
        RawStock {
            symbol,
            date,
            close,
            high,
            low,
            open,
            volume,
            market_time_frame
        }
    }

    pub fn candle_body_size(open: f32, close: f32) -> f32 {
        (open - close).abs()
    }

    pub fn calculate_if_green_candle(open: f32, close: f32) -> bool {
        open < close
    }

    pub fn to_document(&self) -> Document {
        doc! {
            "symbol": self.symbol.clone(),
            "date": self.date.clone(),
            "close": self.close.clone(),
            "high": self.high.clone(),
            "low": self.low.clone(),
            "open": self.open.clone(),
            "volume": self.volume.clone(),
            "market_time_frame": self.market_time_frame.clone().to_string()
        }
    }

    pub async fn fetch_raw_stocks(symbol: String, start_trade_date: String, end_trade_date: String, market_time_frame: TimeFrame) -> Option<Vec<RawStock>>{
        let raw_stock_collection = RawStock::get_stock_collection(symbol.clone()).await;

        let filter = doc!{
            "date": {
                "$gte": start_trade_date.clone(), "$lte": end_trade_date.clone()
            },
            "symbol": symbol.clone(),
            "market_time_frame": market_time_frame.clone().to_string()
    };
        
        // println!("raw_stock_collection: {:?}", raw_stock_collection);
        // println!("filter: {:?}", filter);

        let options = FindOptions::builder().build();

        let cursor = raw_stock_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    // println!("Successfully fetched raw stocks {:?}", Some(data)) );
                    Some(data) 
                }
                Err(e) => {
                    println!("Cursor Error fetch_orders: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error fetch_orders: {}", e);
                None
            }
        }
    }

    pub async fn get_stock_collection(symbol: String) -> Collection<RawStock>{
        let db = mongodb_connection::fetch_db_connection().await;
        let collection = db.collection::<RawStock>(symbol.to_lowercase().as_str());
        collection
    }


}

pub struct RawStockLedger {
    pub raw_stocks: Vec<RawStock>
}

impl RawStockLedger {
    pub fn new() -> RawStockLedger {
        RawStockLedger {
            raw_stocks: Vec::new(),
        }
    }

    pub fn get_raw_stocks(&self) -> &Vec<RawStock> {
        &self.raw_stocks
    }

    pub fn add_raw_stock(&mut self, raw_stock: RawStock) {
        self.raw_stocks.push(raw_stock);
    }
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[allow(dead_code, unused_variables)]
// pub struct BacktestRawStock {
//     pub symbol: String,
//     pub date: String,
//     pub close: f32,
//     pub high: f32,
//     pub low: f32,
//     pub open: f32,
//     pub volume: i32,
//     pub market_time_frame: TimeFrame,
//     pub pnl_configuration_id: String
// }