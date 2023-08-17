
use crate::common::enums::{AlgoTypes, TradeType, TimeFrame};
use crate::common::number_parser::return_2_precision_for_float;
use crate::common::raw_stock::RawStock;
use crate::common::date_parser;
use crate::order_manager::trade_signal_keeper::TradeSignal;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
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

    pub async fn calculate_and_add_ledger(&mut self, stock: &RawStock, hammer_candle_collection: Collection<HammerCandle>) -> Option<TradeSignal> {
        
        let (is_hammer_candle, calculated_body_size, is_green_candle) =
        HammerPatternUtil::calculate_candle_metadata(stock.open, stock.high, stock.low, stock.close);

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
    fn calculate_candle_metadata(
        open: f32,
        high: f32,
        low: f32,
        close: f32,
    ) -> (bool, f32, bool) {
        let calculated_body_size: f32 = RawStock::candle_body_size(open, close);
        let is_hammer_candle =
        HammerPatternUtil::calculate_hammer_candle(calculated_body_size, open, high, low, close);
        let is_green_candle = RawStock::calculate_if_green_candle(open, close);

        (
            is_hammer_candle,
            calculated_body_size,
            is_green_candle,
        )
    }

    fn calculate_hammer_candle(
        calculated_body_size: f32,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
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
        if body_to_full_candle_ratio <= 0.25 {
            lower_wick > (2.1 * upper_wick)
        } else {
            lower_wick_to_body_ratio >= 1.75
                && upper_wick_to_body_ratio <= 1.5
                && (lower_wick > (2.0 * upper_wick))
        }
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

        let (trade_position_type, entry_price) = match previous_hammer_candle.is_green_candle {
            true => (TradeType::Long, return_2_precision_for_float(previous_hammer_candle.close*0.95)), //95% of the close price can be a good entry point
            false => (TradeType::Long, return_2_precision_for_float(previous_hammer_candle.open*0.95)) //95% of the open price can be a good entry point
        }; //hammer candle always going to give long trades

        //TODO: read current market state, analyse and decide whether to take the trade or not
        //like: if market is in downtrend, then don't take the trade or 5 EMA is below 20 EMA, then don't take the trade, etc...

        // let candle = previous_hammer_candle.clone();
        if entry_price > 0.0 {
            let trade_sl = return_2_precision_for_float(entry_price*0.95); //5% SL
            let trade_target = return_2_precision_for_float(entry_price*1.10); //10% Target
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
