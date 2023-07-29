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
}