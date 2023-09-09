use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::common::enums::TimeFrame;
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