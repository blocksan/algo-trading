use colored::*;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use crate::config::mongodb_connection;
use crate::{HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE, HAMMER_RED_CANDLES_COUNT_THRESHOLD, HAMMER_MAX_DROP_THRESHOLD_VALUE, HAMMER_MAX_DROP_CANDLE_COUNT, HAMMER_SL_MARGIN_POINTS, HAMMER_TARGET_MARGIN_MULTIPLIER};
use crate::common::enums::{AlgoTypes, TradeType, TimeFrame};
use crate::common::raw_stock::RawStock;
use crate::common::redis_client::RedisClient;
use crate::common::date_parser;
use crate::common::utils::current_market_state_cache_key_formatter;
use crate::data_consumer::current_market_state::CurrentMarketState;
use crate::data_consumer::previous_volatility_in_stocks::previous_drop_in_stock;
use crate::order_manager::trade_signal_keeper::TradeSignal;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
// const HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE: f32 = 0.0025; //0.25% as a fraction
// const HAMMER_RED_CANDLES_COUNT_THRESHOLD: i32 = 3;
// const HAMMER_MAX_DROP_THRESHOLD_VALUE: f32 = 20.0; //TODO: configure these values based on the index or stocks
// const HAMMER_MAX_DROP_CANDLE_COUNT: usize = 2;
// const HAMMER_SL_MARGIN_POINTS : f32 = 1.0; //2 points
// const HAMMER_TARGET_MARGIN_MULTIPLIER : f32 = 1.5; //1.5 times of the lower wick

#[derive(Deserialize, Debug)]
pub struct HammerCandleBodyParams{
    pub start_trade_date: String,
    pub end_trade_date: String,
    pub symbol: Option<String>,
    pub market_time_frame: Option<TimeFrame>,
    pub is_green_candle: Option<bool>,
    pub volume: Option<i32>,
    pub volume_operator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HammerCandle {
    pub symbol: String,
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: i32,
    pub market_time_frame: TimeFrame,
    pub is_green_candle: bool,
    pub is_hammer: bool,
    pub body_size_ratio: f32,
    pub created_at: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    // pub _id : String, //MongoDB ID => Initialized with empty string, will be updated when inserted into DB
}
impl HammerCandle {
    pub fn new(
        symbol: String,
        date: String,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        volume: i32,
        market_time_frame: TimeFrame,
        is_green_candle: bool,
        is_hammer: bool,
        body_size_ratio: f32,
        created_at: String,
        id: ObjectId,
    ) -> HammerCandle {
        HammerCandle {
            symbol,
            date,
            open,
            high,
            low,
            close,
            volume,
            market_time_frame,
            is_green_candle,
            is_hammer,
            body_size_ratio,
            created_at,
            id
        }
    }

    pub fn update_hammer(&mut self, is_hammer: bool) {
        self.is_hammer = is_hammer;
    }

    pub fn update_body_size_ratio(&mut self, body_size_ratio: f32) {
        self.body_size_ratio = body_size_ratio;
    }

    pub fn update_is_green_candle(&mut self, is_green_candle: bool) {
        self.is_green_candle = is_green_candle;
    }

    pub async fn fetch_hammer_candles(hammer_candles_body_params: HammerCandleBodyParams) -> Option<Vec<HammerCandle>>{
        let hammer_collection = HammerCandle::get_hammer_candles_collection().await; 
        let HammerCandleBodyParams {
            start_trade_date,
            end_trade_date,
            symbol,
            is_green_candle,
            market_time_frame,
            volume,
            volume_operator
        } = hammer_candles_body_params;
        let mut filter = doc! {
            "date": {
                "$gte": start_trade_date,
                "$lte": end_trade_date
            },
        }; 

        if is_green_candle.is_some() {
            filter.insert("is_green_candle", is_green_candle.unwrap());
        }
    
        if market_time_frame.is_some() {
            filter.insert("market_time_frame", market_time_frame.unwrap().to_string());
        }

        if volume.is_some() {
            if volume_operator.is_some() && volume_operator.unwrap() == "$gte"{
                filter.insert( "volume", doc!{
                    "$gte": volume.unwrap()
                });
            }else {
                filter.insert( "volume", doc!{
                    "$lte": volume.unwrap()
                });
            }
        }

        if symbol.is_some() {
            filter.insert("symbol", symbol.unwrap());
        }

        let options = FindOptions::builder().build();

        // let cursor = pnl_configuration_collection.find(filter, options).await?.try_collect::<Vec<_>>().await?;
        let cursor = hammer_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    // println!("Successfully fetched PnL configuration {:?}", pnl_configuration_found);
                    Some(data) 
                }
                Err(e) => {
                    println!("Cursor Error fetch_hammer_candles: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error fetch_hammer_candles: {}", e);
                None
            }
        }
    }

    pub async fn get_hammer_candles_collection() -> Collection<HammerCandle>{
        let db = mongodb_connection::fetch_db_connection().await;
        let hammer_candle_collection_name = "hammer_candles";
        let hammer_candle_collection = db.collection::<HammerCandle>(hammer_candle_collection_name);
        return hammer_candle_collection
    }


    
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HammerPatternUtil {
    pub hammer_pattern_ledger: Vec<HammerCandle>,
}

impl HammerPatternUtil {
    pub fn new() -> HammerPatternUtil {
        HammerPatternUtil {
            hammer_pattern_ledger: Vec::new(),
        }
    }

    fn add_into_hammer_pattern_ledger(&mut self, candle: HammerCandle) -> () {
        self.hammer_pattern_ledger.push(candle);
    }

    pub fn fetch_hammer_pattern_ledger(&self) -> Vec<HammerCandle> {
        self.hammer_pattern_ledger.clone()
    }

    pub async fn calculate_and_add_ledger(&mut self, stock: &RawStock) -> Option<TradeSignal> {
        
        let (is_hammer_candle, calculated_body_size, is_green_candle) =
        HammerPatternUtil::calculate_candle_metadata(stock).await;


        if is_hammer_candle {
            let hammer_candle = HammerCandle::new(
                stock.symbol.clone(),
                stock.date.clone(),
                stock.open,
                stock.high,
                stock.low,
                stock.close,
                stock.volume,
                stock.market_time_frame.clone(),
                is_green_candle,
                is_hammer_candle,
                calculated_body_size,
                date_parser::new_current_date_time_in_desired_stock_datetime_format(),
                ObjectId::new()
            );
            let hammer_candle_collection = HammerCandle::get_hammer_candles_collection().await;
            match hammer_candle_collection.insert_one(hammer_candle.clone(), None).await{
                Ok(_result) => {
                    // println!("Hammer candle inserted into the database {:?}", result);
                },
                Err(e) => println!("Error while inserting hammer candle into the database => {:?}", e)
            }
            self.add_into_hammer_pattern_ledger(hammer_candle);
            self.analyse_and_create_trading_signal()
        }else{
            None
        }
        
    }
    async fn calculate_candle_metadata(
        stock: &RawStock,
    ) -> (bool, f32, bool) {
        let open = stock.open;
        let high = stock.high;
        let low = stock.low;
        let close = stock.close;
        let calculated_body_size: f32 = RawStock::candle_body_size(open, close);
        let is_hammer_candle =
        HammerPatternUtil::calculate_hammer_candle(calculated_body_size, open, high, low, close, stock).await;

        let is_green_candle = RawStock::calculate_if_green_candle(open, close);

        (
            is_hammer_candle,
            calculated_body_size,
            is_green_candle,
        )
    }

    async fn calculate_hammer_candle(
        calculated_body_size: f32,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        stock: &RawStock
    ) -> bool {
        let lower_wick = if open > close {
            close - low
        } else {
            open - low
        };
        let upper_wick = if open > close {
            high - open
        } else {
            high - close
        };

        let lower_wick_to_body_ratio = lower_wick / calculated_body_size;
        let upper_wick_to_body_ratio = upper_wick / calculated_body_size;

        let full_candle_height = high - low;
        let body_to_full_candle_ratio = calculated_body_size / full_candle_height;

        // print!("lower_wick_to_body_ratio => {:?} ", lower_wick_to_body_ratio);
        // print!("upper_wick_to_body_ratio => {:?} ", upper_wick_to_body_ratio);
        // println!("calculated_body_size => {:?}", calculated_body_size);
        // println!("lower_wick => {:?}", lower_wick);
        // println!("upper_wick => {:?}", upper_wick);
        // println!("full_candle_height => {:?}", full_candle_height);
        // println!("body_to_full_candle_ratio => {:?}", body_to_full_candle_ratio);
        //Pattern 1: Condition for the pure Hammer Candle
        let mut standalone_hammer = if body_to_full_candle_ratio <= 0.25 {
            lower_wick > (2.1 * upper_wick)
        } else {
            lower_wick_to_body_ratio >= 1.75
                && upper_wick_to_body_ratio <= 1.5
                && (lower_wick > (2.0 * upper_wick))
        };

        if standalone_hammer {
            println!("");
            println!("-----*****----- {}", "Standalone Hammer Candle found".green());
            println!("{:?}", stock);
            println!("-----*****-----");
            println!("");
        }

        let mut hammer_around_support = false;
        let mut hammer_after_drop = false;
        let mut hammer_after_red_candles = false;
        let redis_client = RedisClient::get_instance();
        let trade_date_only = date_parser::return_only_date_from_datetime(stock.date.as_str());
        let current_market_state_cache_key = current_market_state_cache_key_formatter(trade_date_only.as_str(), stock.symbol.as_str(), &stock.market_time_frame);

        let current_market_state_option = CurrentMarketState::fetch_previous_market_state(&current_market_state_cache_key,redis_client).await;
        
        if current_market_state_option.is_some(){
            let current_market_state = current_market_state_option.unwrap();
            //Pattern 2: Condition for the Hammer Candle above the Support Line
            for support_line in current_market_state.support.iter().rev() {
                    let support_price = *support_line;
                    let max_allowable_difference = support_price * *HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE;
                    let absolute_differnce = (support_price - low).abs();

                    if absolute_differnce <= max_allowable_difference && support_price < low {
                        hammer_around_support = true;
                        println!("");
                        println!("-----*****-----");
                        println!("Hammer Candle found above the support line");
                        println!("{:?}", stock);
                        println!("Support Price => {:?}", support_price);
                        println!("Absolute Difference => {}", format!("{}",absolute_differnce).yellow());
                        println!("-----*****-----");
                        println!("");
                        break;
                    }
                
            }

            //Pattern 3: Condition for the Hammer Candle after 150 points drop on BankNifty or 40 points drop on Nifty/FINNIFTY
            let max_drop_threshold = *HAMMER_MAX_DROP_THRESHOLD_VALUE; // BANKNIFTY //TODO: created configuration map from where drop_threshold value can be read based on the stock
            let max_drop_candle_count = *HAMMER_MAX_DROP_CANDLE_COUNT;
            let valid_drop = previous_drop_in_stock(&current_market_state.raw_stocks, max_drop_candle_count, max_drop_threshold);

            hammer_after_drop = valid_drop;

            if hammer_after_drop {
                println!("");
                println!("-----*****----- ");
                println!("found valid {} points drop", format!("{}",max_drop_threshold).red());
                println!("{:?}", stock);
                println!("-----*****-----");
                println!("");
                if !standalone_hammer && lower_wick_to_body_ratio >= 1.0 && upper_wick_to_body_ratio < 0.3 {
                    standalone_hammer = true;
                    println!("");
                    println!("-----*****----- ");
                    println!("Standalone Hammer Candle found after {} points drop", format!("{}",max_drop_threshold).red());
                    println!("{:?}", stock);
                    println!("-----*****-----");
                    println!("");
                }
            }

            //Pattern 4: Condition for the Hammer Candle after continuous 3 red candles
            hammer_after_red_candles = current_market_state.last_consecutive_red_candle_count >=  *HAMMER_RED_CANDLES_COUNT_THRESHOLD;
            if hammer_after_red_candles{
                println!("");
                println!("-----*****-----");
                println!("Hammer Candle found after continuous {} red candles", format!("{}", *HAMMER_RED_CANDLES_COUNT_THRESHOLD).red());
                println!("{:?}", stock);
                println!("-----*****-----");
                println!("");
            }
                
         }
        
        standalone_hammer && (hammer_around_support || hammer_after_drop || hammer_after_red_candles)

    }

    
    pub fn analyse_and_create_trading_signal(&mut self) -> Option<TradeSignal> {
        let previous_hammer_candle_exists = self.hammer_pattern_ledger.last();
        if previous_hammer_candle_exists.is_none()  {
            return None;
        }   

        let previous_hammer_candle = previous_hammer_candle_exists.unwrap();

        if date_parser::new_current_date_time_in_desired_stock_datetime_format() < previous_hammer_candle.date  {
            return None;
        }

        let (trade_position_type, entry_price, trade_sl, trade_target) = match previous_hammer_candle.is_green_candle {
            true => {
                let lower_wick = previous_hammer_candle.open-previous_hammer_candle.low;
                let entry_price = previous_hammer_candle.close-(*HAMMER_SL_MARGIN_POINTS);
                let trade_sl = entry_price-(lower_wick+(*HAMMER_SL_MARGIN_POINTS));
                let trade_target = entry_price+(lower_wick*(*HAMMER_TARGET_MARGIN_MULTIPLIER));
                (TradeType::Long, entry_price, trade_sl, trade_target)
            }, //2 points below the close price can be a good entry point
                // return_2_precision_for_float(previous_hammer_candle.close*0.95)), //95% of the close price can be a good entry point
            false => {
                let lower_wick = previous_hammer_candle.close-previous_hammer_candle.low;
                let entry_price = previous_hammer_candle.open-(*HAMMER_SL_MARGIN_POINTS);
                let trade_sl = entry_price-(lower_wick+(*HAMMER_SL_MARGIN_POINTS));
                let trade_target = entry_price+((lower_wick+(*HAMMER_SL_MARGIN_POINTS))*(*HAMMER_TARGET_MARGIN_MULTIPLIER));
                (TradeType::Long, entry_price, trade_sl, trade_target)
            }
                // return_2_precision_for_float(previous_hammer_candle.open*0.95)) //95% of the open price can be a good entry point
        }; //hammer candle always going to give long trades

        //TODO: read current market state, analyse and decide whether to take the trade or not
        //like: if market is in downtrend, then don't take the trade or 5 EMA is below 20 EMA, then don't take the trade, etc...

        // let candle = previous_hammer_candle.clone();
        if entry_price > 0.0 {
            // let trade_sl = return_2_precision_for_float(entry_price*0.95); //5% SL
            // let trade_target = return_2_precision_for_float(entry_price*1.10); //10% Target
            // self.hammer_pattern_ledger.pop();
            match TradeSignal::create_trade_signal(previous_hammer_candle.symbol.clone(), previous_hammer_candle.date.clone(), previous_hammer_candle.close, previous_hammer_candle.high,previous_hammer_candle. low, previous_hammer_candle.open, previous_hammer_candle.volume,previous_hammer_candle.market_time_frame.clone(),trade_position_type, AlgoTypes::HammerPatternAlgo, entry_price, trade_sl, trade_target, previous_hammer_candle.id) {
                Some(trade_signal) => {
                    Some(trade_signal)
                },
                None => None
            }
            
        }else{
            None
        }

    }


}

// pub fn tick_receiver(stock: RawStock) {
    

    

    

    

    // for stock in stock_5_min_keeper.iter_mut() {
    //     // println!("Date => {:?}", stock.date);
    //     // println!("Candle Metadata START => {:?}", stock.date);
    //     let (is_hammer, body_size_ratio, is_green_candle) =
    //         calculate_candle_metadata(stock.open, stock.high, stock.low, stock.close);
    //     if is_hammer {
    //         // println!("Hammer star found");
    //         stock.update_hammer(is_hammer);
    //         // println!("{:?}", stock);
    //     }

    //     stock.update_body_size_ratio(body_size_ratio);
    //     stock.update_is_green_candle(is_green_candle);

    //     // println!("Candle Metadata END");
    // }
// }
