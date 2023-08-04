use std::sync::Mutex;

use crate::common::{enums::{TimeFrame, MarketTrend}, raw_stock::{RawStock, RawStockLedger}, date_parser, redis_client::RedisClient, utils::current_market_state_cache_key_formatter};
use mongodb::{Collection, Database, options::{UpdateOptions, FindOneOptions}, bson::{doc, Document}};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMarketState {

    pub market_time_frame: TimeFrame,
    pub previous_candle_market_trend: MarketTrend,
    pub current_candle_market_trend: MarketTrend,
    pub current_sma: f32,  // Simple Moving Average

    pub previous_candle_open: f32,
    pub previous_candle_high: f32,
    pub previous_candle_low: f32,
    pub previous_candle_close: f32,
    pub previous_candle_volume: i32,

    pub current_candle_open: f32,
    pub current_candle_high: f32,
    pub current_candle_low: f32,
    pub current_candle_close: f32,
    pub current_candle_volume: i32,

    pub last_consecutive_green_candle_count: i32,
    pub last_consecutive_red_candle_count: i32,

    pub symbol: String,
    pub trade_date: String,
    pub last_updated_at: String,
    pub cache_key: String,

}

impl From<CurrentMarketState> for Document{
    fn from(current_market_state: CurrentMarketState) -> Self {
        doc! {
            "market_time_frame": current_market_state.market_time_frame.to_string(),
            "previous_candle_market_trend": current_market_state.previous_candle_market_trend.to_string(),
            "current_candle_market_trend": current_market_state.current_candle_market_trend.to_string(),
            "current_sma": current_market_state.current_sma,
            "previous_candle_open": current_market_state.previous_candle_open,
            "previous_candle_high": current_market_state.previous_candle_high,
            "previous_candle_low": current_market_state.previous_candle_low,
            "previous_candle_close": current_market_state.previous_candle_close,
            "previous_candle_volume": current_market_state.previous_candle_volume,
            "current_candle_open": current_market_state.current_candle_open,
            "current_candle_high": current_market_state.current_candle_high,
            "current_candle_low": current_market_state.current_candle_low,
            "current_candle_close": current_market_state.current_candle_close,
            "current_candle_volume": current_market_state.current_candle_volume,
            "last_consecutive_green_candle_count": current_market_state.last_consecutive_green_candle_count,
            "last_consecutive_red_candle_count": current_market_state.last_consecutive_red_candle_count,
            "symbol": current_market_state.symbol,
            "trade_date": current_market_state.trade_date,
            "last_updated_at": current_market_state.last_updated_at,
            "cache_key": current_market_state.cache_key,
        }
    }
}

// impl UpdateModifications for CurrentMarketState{
//     fn update(&self, doc: &mut Document)-> Result<()>{
//         doc.insert("market_time_frame", self.market_time_frame.to_string());
//         doc.insert("previous_candle_market_trend", self.previous_candle_market_trend.to_string());
//         doc.insert("current_candle_market_trend", self.current_candle_market_trend.to_string());
//         doc.insert("current_sma", self.current_sma);
//         doc.insert("previous_candle_open", self.previous_candle_open);
//         doc.insert("previous_candle_high", self.previous_candle_high);
//         doc.insert("previous_candle_low", self.previous_candle_low);
//         doc.insert("previous_candle_close", self.previous_candle_close);
//         doc.insert("previous_candle_volume", self.previous_candle_volume);
//         doc.insert("current_candle_open", self.current_candle_open);
//         doc.insert("current_candle_high", self.current_candle_high);
//         doc.insert("current_candle_low", self.current_candle_low);
//         doc.insert("current_candle_close", self.current_candle_close);
//         doc.insert("current_candle_volume", self.current_candle_volume);
//         doc.insert("last_consecutive_green_candle_count", self.last_consecutive_green_candle_count);
//         doc.insert("last_consecutive_red_candle_count", self.last_consecutive_red_candle_count);
//         doc.insert("symbol", self.symbol);
//         doc.insert("trade_date", self.trade_date);
//         doc.insert("last_updated_at", self.last_updated_at);
//         Ok(())
//     }
// }
#[allow(non_snake_case, dead_code, unused_variables)]

impl CurrentMarketState {
    pub fn new(
        market_time_frame: TimeFrame,
        previous_candle_market_trend: MarketTrend,
        current_candle_market_trend: MarketTrend,
        current_sma: f32,
        previous_candle_open: f32,
        previous_candle_high: f32,
        previous_candle_low: f32,
        previous_candle_close: f32,
        previous_candle_volume: i32,
        current_candle_open: f32,
        current_candle_high: f32,
        current_candle_low: f32,
        current_candle_close: f32,
        current_candle_volume: i32,
        last_consecutive_green_candle_count: i32,
        last_consecutive_red_candle_count: i32,
        symbol: String,
        trade_date: String,
        last_updated_at: String,
        cache_key: String,
    ) -> CurrentMarketState {
        CurrentMarketState {
            market_time_frame,
            previous_candle_market_trend,
            current_candle_market_trend,
            current_sma,
            previous_candle_open,
            previous_candle_high,
            previous_candle_low,
            previous_candle_close,
            previous_candle_volume,
            current_candle_open,
            current_candle_high,
            current_candle_low,
            current_candle_close,
            current_candle_volume,
            last_consecutive_green_candle_count,
            last_consecutive_red_candle_count,
            symbol,
            trade_date,
            last_updated_at,
            cache_key
        }
    }

    pub fn update_current_candle_market_trend(&mut self, current_state: CurrentMarketState) -> Self {
        Self {
            market_time_frame: current_state.market_time_frame,
            previous_candle_market_trend: current_state.previous_candle_market_trend,
            current_candle_market_trend: current_state.current_candle_market_trend,
            current_sma: current_state.current_sma,
            previous_candle_open: current_state.previous_candle_open,
            previous_candle_high: current_state.previous_candle_high,
            previous_candle_low: current_state.previous_candle_low,
            previous_candle_close: current_state.previous_candle_close,
            previous_candle_volume: current_state.previous_candle_volume,
            current_candle_open: current_state.current_candle_open,
            current_candle_high: current_state.current_candle_high,
            current_candle_low: current_state.current_candle_low,
            current_candle_close: current_state.current_candle_close,
            current_candle_volume: current_state.current_candle_volume,
            last_consecutive_green_candle_count: current_state.last_consecutive_green_candle_count,
            last_consecutive_red_candle_count: current_state.last_consecutive_red_candle_count,
            symbol: current_state.symbol,
            trade_date: current_state.trade_date,
            last_updated_at: current_state.last_updated_at,
            cache_key: current_state.cache_key
        }
    }

    fn to_document(&self) -> Document {
        doc!{
            "market_time_frame": self.market_time_frame.to_string(),
            "previous_candle_market_trend": self.previous_candle_market_trend.to_string(),
            "current_candle_market_trend": self.current_candle_market_trend.to_string(),
            "current_sma": self.current_sma,
            "previous_candle_open": self.previous_candle_open,
            "previous_candle_high": self.previous_candle_high,
            "previous_candle_low": self.previous_candle_low,
            "previous_candle_close": self.previous_candle_close,
            "previous_candle_volume": self.previous_candle_volume,
            "current_candle_open": self.current_candle_open,
            "current_candle_high": self.current_candle_high,
            "current_candle_low": self.current_candle_low,
            "current_candle_close": self.current_candle_close,
            "current_candle_volume": self.current_candle_volume,
            "last_consecutive_green_candle_count": self.last_consecutive_green_candle_count,
            "last_consecutive_red_candle_count": self.last_consecutive_red_candle_count,
            "symbol": self.symbol.to_string(),
            "trade_date": self.trade_date.to_string(),
            "last_updated_at": self.last_updated_at.to_string(),
            "cache_key": self.cache_key.to_string()
        }
    }

    pub async fn calculate_market_state(stock: &RawStock, time_frame: TimeFrame, current_market_state_collection: &Collection<CurrentMarketState>, redis_client: &Mutex<RedisClient>, raw_stock_ledger: &RawStockLedger, database_instance: Database) {

        let trade_date_only = date_parser::return_only_date_from_datetime(stock.date.as_str());
        let current_market_state_cache_key = current_market_state_cache_key_formatter(trade_date_only.as_str(), stock.symbol.as_str(), &stock.market_time_frame);
        let filter = doc! {"cache_key": current_market_state_cache_key.clone() };
        let options = FindOneOptions::builder().build();
        let previous_market_state = match current_market_state_collection.find_one(filter, options).await {
            Ok(Some(data)) => {
                // println!("Data fetched from current_market_stats for key => {}", current_market_state_cache_key);
                Some(data)
            },
            Ok(None) => None,
            Err(e) => {
                println!("Error while fetching the data from MongoDB => {:?}", e);
                None
            }
        };

         let current_market_state = match time_frame {
            TimeFrame::OneMinute => {
                None
                // Self::calculate_market_state_for_oneminute(stock)
            },
            TimeFrame::ThreeMinutes => {
                None
                //  Self::calculate_market_state_for_threeminutes(stock)
            },
            TimeFrame::FiveMinutes => {
                Self::calculate_market_state_for_fiveminutes(stock, redis_client, raw_stock_ledger, current_market_state_cache_key, previous_market_state) 
            },
            TimeFrame::FifteenMinutes => {
                None
                // Self::calculate_market_state_for_fifteenminutes(stock)
            },
            TimeFrame::OneDay => {
                None
                // Self::calculate_market_state_for_oneday(stock)
            },
            TimeFrame::OneWeek => {
                None
                // Self::calculate_market_state_for_oneweek(stock)
            },
            TimeFrame::OneMonth => {
                None
                // Self::calculate_market_state_for_onemonth(stock)
            },
            TimeFrame::OneYear => {
                None
                // Self::calculate_market_state_for_oneyear(stock)
            },
            _ => {
                None
            }
        };

        match current_market_state {
            Some(current_market_state) => {
                let filter = doc! {"cache_key": current_market_state.cache_key.clone() };
                let options = UpdateOptions::builder().upsert(true).build();
                let document = doc!{"$set":current_market_state.to_document()};
                match current_market_state_collection.update_one(filter,  document, options).await {
                    Ok(_) => {
                        // println!("Successfully inserted a current_market_state into the collection");
                    },
                    Err(e) => {
                        println!("Error while inserting a current_market_state into the collection: {:?} error {:?}", current_market_state,e);
                    }
                }
            },
            None => {
                println!("No market state found for the stock: {}", stock.symbol);
            }
        }
    }

    fn calculate_market_state_for_oneminute(stock: &RawStock) -> Option<CurrentMarketState>{
        Some(CurrentMarketState::new (
            TimeFrame::OneMinute,
            MarketTrend::Bearish,
            MarketTrend::Bearish,
            0.0,
            100.0,
            200.0,
            50.0,
            150.0,
            2000,
            200.0,
            300.0,
            150.0,
            250.0,
            3000,
            2,
            2,
            "ADANIGREEN".to_owned(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            "cache_key".to_owned() //TODO: generate cache key
        ))
    }
    fn calculate_market_state_for_threeminutes(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_fiveminutes(stock: &RawStock, redis_client: &Mutex<RedisClient>, raw_stock_ledger: &RawStockLedger, current_market_state_cache_key: String, previous_market_state_db: Option<CurrentMarketState>)-> Option<CurrentMarketState>{
        
        

        let previous_market_state =  match previous_market_state_db {
            Some(previous_market_state) => Some(previous_market_state),
            None => {
                match redis_client.lock().unwrap().get_data(current_market_state_cache_key.as_str()) {
                    Ok(data) => {
                        // println!("Data fetched from Redis for key => {}", current_market_state_cache_key);
                        let deserialised_data = serde_json::from_str::<CurrentMarketState>(data.as_str()).unwrap();
                        Some(deserialised_data)
                    }
                    Err(e) => {
                        println!("Error while fetching the data from Redis => {:?}", e);
                        //fetch from the mongodb
                        None
                    }
                }
            }
        };

        //TODO: find the logic for calculating the current day market trend based on SMA and EMA or other indicators
        let sma_window_size = 9; //TimeFrame::FiveMinutes as i32;
        let (current_sma, current_candle_market_trend) = Self::identify_market_trend_SMA(&raw_stock_ledger.raw_stocks, stock, sma_window_size);

        let updated_market_state = match previous_market_state {
            Some(previous_market_state) => {
                let mut update_required = false;
                let new_stock_low = if stock.low < previous_market_state.current_candle_low {
                    update_required = true;
                    stock.low
                } else {
                    previous_market_state.current_candle_low
                };
                let new_stock_high = if stock.high > previous_market_state.current_candle_high {
                    update_required = true;
                    stock.high
                } else {
                    previous_market_state.current_candle_high
                };

                let new_stock_close = if stock.close < previous_market_state.current_candle_close {
                    update_required = true;
                    stock.close
                } else {
                    previous_market_state.current_candle_close
                };

                let new_stock_volume = previous_market_state.current_candle_volume + stock.volume;
                
                let last_consecutive_green_candle_count = if stock.close > stock.open {
                    update_required = true;
                    previous_market_state.last_consecutive_green_candle_count + 1
                } else {
                    0
                };

                let last_consecutive_red_candle_count = if stock.close < stock.open {
                    update_required = true;
                    previous_market_state.last_consecutive_red_candle_count + 1
                } else {
                    0
                };

                
                // println!("last_consecutive_green_candle_count => {}", last_consecutive_green_candle_count);
                // println!("last_consecutive_red_candle_count => {}", last_consecutive_red_candle_count);
                // println!("current_candle_market_trend => {:?}", current_candle_market_trend);
                // println!("current_SMA => {}", current_sma);
                // println!("current_candle_close => {}", stock.close.clone());
                // println!("update_required => {}", update_required);
                // println!("previous_market_state => {:?}", previous_market_state);

                if update_required {
                    let current_market_state = CurrentMarketState::new(
                        stock.market_time_frame.clone(),
                        MarketTrend::Bearish,
                        current_candle_market_trend,
                        current_sma,
                        previous_market_state.current_candle_open,
                        previous_market_state.current_candle_high,
                        previous_market_state.current_candle_low,
                        previous_market_state.current_candle_close,
                        previous_market_state.current_candle_volume,
                        previous_market_state.current_candle_open,
                        new_stock_high,
                        new_stock_low,
                        new_stock_close,
                        new_stock_volume,
                        last_consecutive_green_candle_count,
                        last_consecutive_red_candle_count,
                        stock.symbol.to_owned(),
                        previous_market_state.trade_date,
                        date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                        current_market_state_cache_key.clone(),
                    );
                    Some(current_market_state)
                }else{
                    None
                }

            }
            None => {
                Some(CurrentMarketState::new(
                    stock.market_time_frame.clone(),
                    MarketTrend::Bearish, //TODO:calculate it => adding dummy data now
                    current_candle_market_trend,
                    current_sma,
                    100.0,
                    200.0,
                    50.0,
                    150.0,
                    2000,
                    stock.open,
                    stock.high,
                    stock.low,
                    stock.close,
                    stock.volume,
                    if stock.close > stock.open {1} else {0}, //TODO:calculate it => adding dummy data now
                    if stock.open > stock.close {1} else {0}, //TODO: calculate it => adding dummy data now
                    stock.symbol.to_owned(),
                    date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                    date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                    current_market_state_cache_key.clone()
                ))
            }
        };

        match updated_market_state {
            Some(updated_market_state) => {
                // println!("Updated market state => {:?}", updated_market_state);
                match redis_client.lock().unwrap().set_data(current_market_state_cache_key.as_str(), serde_json::to_string(&updated_market_state.clone()).unwrap().as_str()) {
                    Ok(_) => {
                        // println!("Data set in Redis for key => {}", current_market_state_cache_key);
                        Some(updated_market_state)
                    }
                    Err(e) => {
                        println!("Error while setting the data in Redis => {:?}", e);
                        None
                    }
                }
            }
            None => None
            
        }
    }
    fn calculate_market_state_for_fifteenminutes(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_oneday(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_oneweek(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_onemonth(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_oneyear(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }

    //simple moving average based market trend identification
    fn identify_market_trend_SMA(raw_stocks: &Vec<RawStock>, stock: &RawStock, sma_window_size: usize)-> (f32, MarketTrend){

        if raw_stocks.len() <= sma_window_size {
            (0.0, MarketTrend::Sideways) // Not enough data points for analysis
        }else{
            let mut prices: Vec<f32> = Vec::new();
            for raw_stock in raw_stocks {
                prices.push(raw_stock.close);
            }
            let sma = prices.windows(sma_window_size).map(|price| price.iter().sum::<f32>() / sma_window_size as f32);
        
            let current_close_price = stock.close;
            let current_sma = sma.last().unwrap_or_default();
        
            if current_close_price > current_sma {
                (current_sma, MarketTrend::Bullish)
            } else if current_close_price < current_sma {
                (current_sma, MarketTrend::Bearish)
            } else {
                (current_sma, MarketTrend::Sideways)
            }
        }
    }
}

