use colored::*;
use std::sync::Mutex;
use crate::{common::{enums::{TimeFrame, MarketTrend}, raw_stock::{RawStock, RawStockLedger}, date_parser, redis_client::RedisClient, utils::current_market_state_cache_key_formatter}, config::mongodb_connection};
use mongodb::{Collection, options::{UpdateOptions, FindOneOptions}, bson::{doc, oid::ObjectId, Document}};
use serde::{Deserialize, Serialize};

use super::support_resistance_fractol::find_support_resistance;
const PIVOT_DEPTH: usize = 3; //4 pivot depth means 3 candle left side and 3 candle right side

#[derive(Deserialize, Debug)]
pub struct CurrentMarketStateBodyParams{
    pub symbol: String,
    pub trade_date: String,
    pub time_frame: TimeFrame
}

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

    #[serde(rename = "_id") ]
    pub id: ObjectId,
    pub created_at: String,
    pub updated_at: String,
    pub cache_key: String,

    pub raw_stocks: Vec<RawStock>,
    pub support: Vec<f32>,
    pub resistance: Vec<f32>

}

// impl From<CurrentMarketState> for Document{
//     fn from(current_market_state: CurrentMarketState) -> Self {
//         doc! {
//             "market_time_frame": current_market_state.market_time_frame.to_string(),
//             "previous_candle_market_trend": current_market_state.previous_candle_market_trend.to_string(),
//             "current_candle_market_trend": current_market_state.current_candle_market_trend.to_string(),
//             "current_sma": current_market_state.current_sma,
//             "previous_candle_open": current_market_state.previous_candle_open,
//             "previous_candle_high": current_market_state.previous_candle_high,
//             "previous_candle_low": current_market_state.previous_candle_low,
//             "previous_candle_close": current_market_state.previous_candle_close,
//             "previous_candle_volume": current_market_state.previous_candle_volume,
//             "current_candle_open": current_market_state.current_candle_open,
//             "current_candle_high": current_market_state.current_candle_high,
//             "current_candle_low": current_market_state.current_candle_low,
//             "current_candle_close": current_market_state.current_candle_close,
//             "current_candle_volume": current_market_state.current_candle_volume,
//             "last_consecutive_green_candle_count": current_market_state.last_consecutive_green_candle_count,
//             "last_consecutive_red_candle_count": current_market_state.last_consecutive_red_candle_count,
//             "symbol": current_market_state.symbol,
//             "created_at": current_market_state.created_at,
//             "updated_at": current_market_state.updated_at,
//             "cache_key": current_market_state.cache_key,
//         }
//     }
// }

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
//         doc.insert("created_at", self.created_at);
//         doc.insert("updated_at", self.updated_at);
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
        id: ObjectId,
        created_at: String,
        updated_at: String,
        cache_key: String,
        raw_stocks: Vec<RawStock>,
        support: Vec<f32>,
        resistance: Vec<f32>
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
            id,
            created_at,
            updated_at,
            cache_key,
            raw_stocks,
            support,
            resistance
        }
    }

    fn to_document(&self) -> Document {
        let raw_stocks_document = self.raw_stocks.iter().map(|raw_stock| raw_stock.to_document()).collect::<Vec<Document>>();
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
            "id": self.id.to_string(),
            "created_at": self.created_at.to_string(),
            "updated_at": self.updated_at.to_string(),
            "cache_key": self.cache_key.to_string(),
            "raw_stocks": raw_stocks_document,
            "support": self.support.clone(),
            "resistance": self.resistance.clone()


        }
    }

    pub async fn calculate_market_state(stock: &RawStock, time_frame: TimeFrame, redis_client: &Mutex<RedisClient>, raw_stock_ledger: &RawStockLedger) {
        println!("Calculating market state for stock_time => {}", format!("{}",stock.date).yellow());
        let trade_date_only = date_parser::return_only_date_from_datetime(stock.date.as_str());
        let current_market_state_cache_key = current_market_state_cache_key_formatter(trade_date_only.as_str(), stock.symbol.as_str(), &stock.market_time_frame);
        
        let previous_market_state = Self::fetch_previous_market_state(current_market_state_cache_key.as_str(), redis_client).await;

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
                Self::calculate_market_state_for_fiveminutes(stock.clone(), raw_stock_ledger, current_market_state_cache_key, previous_market_state) 
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

        Self::push_current_market_state_to_redis_mongo(&current_market_state, redis_client).await;
    }

    fn calculate_market_state_for_oneminute(stock: RawStock) -> Option<CurrentMarketState>{
        Some(CurrentMarketState::new (
            TimeFrame::OneMinute,
            MarketTrend::Sideways,
            MarketTrend::Sideways,
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
            ObjectId::new(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            "cache_key".to_owned(), //TODO: generate cache key
            [stock].to_vec(),
            [0.0].to_vec(),
            [0.0].to_vec()
        ))
    }
    fn calculate_market_state_for_threeminutes(stock: RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_fiveminutes(stock: RawStock, raw_stock_ledger: &RawStockLedger, current_market_state_cache_key: String, previous_market_state: Option<CurrentMarketState>)-> Option<CurrentMarketState>{
         //TODO: find the logic for calculating the current day market trend based on SMA and EMA or other indicators
        let sma_window_size = 9; //TimeFrame::FiveMinutes as i32;
        let (current_sma, current_candle_market_trend) = Self::identify_market_trend_SMA(&raw_stock_ledger.raw_stocks, &stock, sma_window_size);

        match previous_market_state {
            Some(mut previous_market_state) => {
                let new_stock_low = if stock.low < previous_market_state.current_candle_low {
                    stock.low
                } else {
                    previous_market_state.current_candle_low
                };
                let new_stock_high = if stock.high > previous_market_state.current_candle_high {
                    stock.high
                } else {
                    previous_market_state.current_candle_high
                };

                let new_stock_close = if stock.close < previous_market_state.current_candle_close {
                    stock.close
                } else {
                    previous_market_state.current_candle_close
                };

                let new_stock_volume = previous_market_state.current_candle_volume + stock.volume;
                
                let last_consecutive_green_candle_count = if stock.close > stock.open {
                    previous_market_state.last_consecutive_green_candle_count + 1
                } else {
                    0
                };

                let last_consecutive_red_candle_count = if stock.close < stock.open {
                    previous_market_state.last_consecutive_red_candle_count + 1
                } else {
                    0
                };

                previous_market_state.raw_stocks.push(stock.clone());

                let (support, resistance) =find_support_resistance(&previous_market_state.raw_stocks, PIVOT_DEPTH);
                
                // println!("last_consecutive_green_candle_count => {}", last_consecutive_green_candle_count);
                // println!("last_consecutive_red_candle_count => {}", last_consecutive_red_candle_count);
                // println!("current_candle_market_trend => {:?}", current_candle_market_trend);
                // println!("current_SMA => {}", current_sma);
                // println!("current_candle_close => {}", stock.close.clone());
                // println!("update_required => {}", update_required);
                // println!("previous_market_state => {:?}", previous_market_state);
                let update_required = true;

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
                        previous_market_state.id,
                        previous_market_state.created_at,
                        date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                        current_market_state_cache_key.clone(),
                        previous_market_state.raw_stocks,
                        support,
                        resistance
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
                    ObjectId::new(),
                    date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                    date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                    current_market_state_cache_key.clone(),
                    [stock].to_vec(),
                    [0.0].to_vec(),
                    [0.0].to_vec()
                ))
            }
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

    async fn push_current_market_state_to_redis_mongo(current_market_state: &Option<CurrentMarketState>, redis_client: &Mutex<RedisClient>){
        match current_market_state {
            Some(current_market_state) => {
                let current_market_state_collection = Self::get_current_market_state_collection().await;
                let filter = doc! {"cache_key": current_market_state.cache_key.clone() };
                let options = UpdateOptions::builder().upsert(true).build();
                let document = doc!{"$set":current_market_state.to_document()};
                match current_market_state_collection.update_one(filter,  document, options).await {
                    Ok(_) => {
                        // println!("Successfully inserted a current_market_state into the collection");
                    },
                    Err(e) => {
                        println!("Error while updating a current_market_state into the collection: {:?} error {:?}", current_market_state,e);
                    }
                }

                match redis_client.lock().unwrap().set_data(current_market_state.cache_key.as_str(), serde_json::to_string(&current_market_state.clone()).unwrap().as_str()) {
                    Ok(_) => {
                        // println!("Data set in Redis for key => {}", current_market_state_cache_key);
                    }
                    Err(e) => {
                        println!("Error while setting the data in Redis => {:?}", e);
                    }
                }
            },
            None => {
                println!("No current market state");
            }
        }
    }
    pub async fn fetch_previous_market_state(current_market_state_cache_key: &str, redis_client: &Mutex<RedisClient>)->Option<CurrentMarketState>{
        let current_market_state_collection = Self::get_current_market_state_collection().await;
        let filter = doc! {"cache_key": current_market_state_cache_key.clone() };
        let options = FindOneOptions::builder().build();
        let previous_market_state = match redis_client.lock().unwrap().get_data(current_market_state_cache_key) {
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
        };
        if previous_market_state.is_some(){
            previous_market_state
        }else{
            match current_market_state_collection.find_one(filter, options).await {
                Ok(Some(data)) => {
                    // println!("Data fetched from current_market_stats for key => {}", current_market_state_cache_key);
                    Some(data)
                },
                Ok(None) => None,
                Err(e) => {
                    println!("Error while fetching the data from MongoDB => {:?}", e);
                    None
                }
            }
        }
    }

    pub async fn fetch_current_market_states(current_market_state_body_params: CurrentMarketStateBodyParams, only_via_redis: bool)->Option<CurrentMarketState>{
        // current_market_state_cache_key: &str,
        let CurrentMarketStateBodyParams{trade_date, symbol, time_frame} = current_market_state_body_params;
        
        let trade_date_only = date_parser::return_only_date_from_datetime(trade_date.as_str());
        let current_market_state_cache_key = current_market_state_cache_key_formatter(trade_date_only.as_str(), symbol.as_str(), &time_frame);
        let current_market_state_collection = Self::get_current_market_state_collection().await;
        let redis_client = RedisClient::get_instance();
        let previous_market_state = match redis_client.lock().unwrap().get_data(current_market_state_cache_key.as_str()) {
                    Ok(data) => {
                        // println!("Data fetched from Redis for key => {}", current_market_state_cache_key);
                        let deserialised_data = serde_json::from_str::<CurrentMarketState>(data.as_str()).unwrap();
                        Some(deserialised_data)
                    }
                    Err(e) => {
                        println!("Error while fetching the fetch_current_market_states from Redis => {:?}", e);
                        //fetch from the mongodb
                        None
                    }
        };
        if previous_market_state.is_some(){
            previous_market_state
        }else if only_via_redis && previous_market_state.is_none() {
            None
        }else{
            let filter = doc! {"cache_key": current_market_state_cache_key.clone() };
            let options = FindOneOptions::builder().build();
            match current_market_state_collection.find_one(filter, options).await {
                Ok(Some(data)) => {
                    // println!("Data fetched from current_market_stats for key => {}", current_market_state_cache_key);
                    Some(data)
                },
                Ok(None) => None,
                Err(e) => {
                    println!("Error while fetching fetch_current_market_states from MongoDB => {:?}", e);
                    None
                }
            }
        }
    }

    pub async fn get_current_market_state_collection() -> Collection<CurrentMarketState> {
        let db = mongodb_connection::fetch_db_connection().await;
        let current_market_state_collection_name = "current_market_states";
        let current_market_state_collection = db.collection::<CurrentMarketState>(current_market_state_collection_name);
        current_market_state_collection
    }

    
}

