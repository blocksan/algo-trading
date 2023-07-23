
#[path = "../common/mod.rs"] mod common;
use crate::common::enums::TradeType;
use crate::common::number_parser::return_2_precision_for_float;
use crate::common::raw_stock::RawStock;
use crate::common::algo_types::AlgoTypes;
use crate::common::date_parser;
use crate::order_manager::trade_signal_keeper::TradeSignal;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HammerCandle {
    pub symbol: String,
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
    pub is_green_candle: bool,
    pub is_hammer: bool,
    pub body_size_ratio: f32,
    pub identified_at: String,
}
impl HammerCandle {
    pub fn new(
        symbol: String,
        date: String,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        volume: f32,
        is_green_candle: bool,
        is_hammer: bool,
        body_size_ratio: f32,
        identified_at: String,
    ) -> HammerCandle {
        HammerCandle {
            symbol,
            date,
            open,
            high,
            low,
            close,
            volume,
            is_green_candle,
            is_hammer,
            body_size_ratio,
            identified_at,
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

    pub fn calculate_and_add_ledger(&mut self, stock: &RawStock) -> () {
        if self.hammer_pattern_ledger.len() > 0 {
            ()
        }
        let (is_hammer_candle, calculated_body_size_ratio, is_green_candle) =
        HammerPatternUtil::calculate_candle_metadata(stock.open, stock.high, stock.low, stock.close);

        if is_hammer_candle {
            let hammer_pattern = HammerCandle::new(
                stock.symbol.clone(),
                stock.date.clone(),
                stock.open,
                stock.high,
                stock.low,
                stock.close,
                stock.volume,
                is_green_candle,
                is_hammer_candle,
                calculated_body_size_ratio,
                date_parser::new_current_date_time_in_desired_stock_datetime_format()
            );
            self.add_into_hammer_pattern_ledger(hammer_pattern);
            ()
        }
        
    }
    fn calculate_candle_metadata(
        open: f32,
        high: f32,
        low: f32,
        close: f32,
    ) -> (bool, f32, bool) {
        let calculated_body_size_ratio: f32 = RawStock::candle_body_size(open, close);
        let is_hammer_candle =
        HammerPatternUtil::calculate_hammer_candle(calculated_body_size_ratio, open, high, low, close);
        let is_green_candle = RawStock::calculate_if_green_candle(open, close);

        (
            is_hammer_candle,
            calculated_body_size_ratio,
            is_green_candle,
        )
    }

    fn calculate_hammer_candle(
        calculated_body_size_ratio: f32,
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

        let lower_wick_to_body_ratio = lower_wick / calculated_body_size_ratio;
        let upper_wick_to_body_ratio = upper_wick / calculated_body_size_ratio;

        let full_candle_height = high - low;
        let body_to_full_candle_ratio = calculated_body_size_ratio / full_candle_height;

        // print!("lower_wick_to_body_ratio => {:?} ", lower_wick_to_body_ratio);
        // print!("upper_wick_to_body_ratio => {:?} ", upper_wick_to_body_ratio);
        // println!("calculated_body_size_ratio => {:?}", calculated_body_size_ratio);
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

    
    pub fn check_for_trade_opportunity(&mut self) -> Option<TradeSignal> {
        let previous_hammer_candle_exists = self.hammer_pattern_ledger.last();

        if previous_hammer_candle_exists.is_none()  {
            return None;
        }   

        let previous_hammer_candle = previous_hammer_candle_exists.unwrap();

        let trade_position_type = match previous_hammer_candle.is_green_candle {
            true => TradeType::Long,
            false => TradeType::Long
        }; //hammer candle always going to give long trades

        let entry_price = match previous_hammer_candle.is_green_candle {
            true => {
                if date_parser::new_current_date_time_in_desired_stock_datetime_format() > previous_hammer_candle.date  {
                return_2_precision_for_float(previous_hammer_candle.close*1.005)
              } else {
                -1.0
              }
            },
            false => {
             if date_parser::new_current_date_time_in_desired_stock_datetime_format() > previous_hammer_candle.date  {
                return_2_precision_for_float(previous_hammer_candle.open*1.005)
              } else {
                -1.0
              }
            }
        };

        // let candle = previous_hammer_candle.clone();
        if entry_price > 0.0 {
            let trade_sl = return_2_precision_for_float(entry_price*0.95); //5% SL
            let trade_target = return_2_precision_for_float(entry_price*1.10); //10% Target
            // self.hammer_pattern_ledger.pop();
            match HammerPatternUtil::create_trade_signal(previous_hammer_candle.symbol.clone(), previous_hammer_candle.date.clone(), previous_hammer_candle.close, previous_hammer_candle.high,previous_hammer_candle. low, previous_hammer_candle.open, previous_hammer_candle.volume,trade_position_type, AlgoTypes::HammerPatternAlgo, entry_price, trade_sl, trade_target) {
                Some(trade_signal) => {
                    Some(trade_signal)
                },
                None => None
            }
            
        }else{
            None
        }

    }

    fn create_trade_signal(symbol: String, date: String, close: f32, high:f32, low:f32, open:f32, volume:f32, trade_position_type: TradeType, algo_type: AlgoTypes, entry_price: f32, trade_sl: f32, trade_target: f32 ) -> Option<TradeSignal> {
        const QTY:i32 = 10;
        let trade_signal = TradeSignal::new(
            RawStock::new(
                symbol,
                date,
                close,
                high,
                low,
                open,
                volume,
            ),
            trade_position_type,
            algo_type,
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            entry_price,
            trade_sl,
            trade_target,
            QTY,
            entry_price*QTY as f32,
        );
        Some(trade_signal)
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
