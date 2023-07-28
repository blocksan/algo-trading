use crate::common::{enums::{TimeFrame, MarketTrend}, raw_stock::RawStock, date_parser};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMarketState {

    pub market_time_frame: TimeFrame,
    pub previous_day_market_trend: MarketTrend,
    pub current_day_market_trend: MarketTrend,

    pub previous_day_open: f32,
    pub previous_day_high: f32,
    pub previous_day_low: f32,
    pub previous_day_close: f32,
    pub previous_day_volume: f32,

    pub current_day_open: f32,
    pub current_day_high: f32,
    pub current_day_low: f32,
    pub current_day_close: f32,
    pub current_day_volume: f32,

    pub last_consecutive_green_candle_count: i32,
    pub last_consecutive_red_candle_count: i32,

    pub symbol: String,
    pub current_date: String,
    pub last_updated_at: String,

}

impl CurrentMarketState {
    pub fn new(
        market_time_frame: TimeFrame,
        previous_day_market_trend: MarketTrend,
        current_day_market_trend: MarketTrend,
        previous_day_open: f32,
        previous_day_high: f32,
        previous_day_low: f32,
        previous_day_close: f32,
        previous_day_volume: f32,
        current_day_open: f32,
        current_day_high: f32,
        current_day_low: f32,
        current_day_close: f32,
        current_day_volume: f32,
        last_consecutive_green_candle_count: i32,
        last_consecutive_red_candle_count: i32,
        symbol: String,
        current_date: String,
        last_updated_at: String,
    ) -> CurrentMarketState {
        CurrentMarketState {
            market_time_frame,
            previous_day_market_trend,
            current_day_market_trend,
            previous_day_open,
            previous_day_high,
            previous_day_low,
            previous_day_close,
            previous_day_volume,
            current_day_open,
            current_day_high,
            current_day_low,
            current_day_close,
            current_day_volume,
            last_consecutive_green_candle_count,
            last_consecutive_red_candle_count,
            symbol,
            current_date,
            last_updated_at,
        }
    }

    pub fn update_current_day_market_trend(&mut self, current_state: CurrentMarketState) -> Self {
        Self {
            market_time_frame: current_state.market_time_frame,
            previous_day_market_trend: current_state.previous_day_market_trend,
            current_day_market_trend: current_state.current_day_market_trend,
            previous_day_open: current_state.previous_day_open,
            previous_day_high: current_state.previous_day_high,
            previous_day_low: current_state.previous_day_low,
            previous_day_close: current_state.previous_day_close,
            previous_day_volume: current_state.previous_day_volume,
            current_day_open: current_state.current_day_open,
            current_day_high: current_state.current_day_high,
            current_day_low: current_state.current_day_low,
            current_day_close: current_state.current_day_close,
            current_day_volume: current_state.current_day_volume,
            last_consecutive_green_candle_count: current_state.last_consecutive_green_candle_count,
            last_consecutive_red_candle_count: current_state.last_consecutive_red_candle_count,
            symbol: current_state.symbol,
            current_date: current_state.current_date,
            last_updated_at: current_state.last_updated_at,
        }
    }

    pub async fn calculate_market_state(stock: &RawStock, time_frame: TimeFrame, current_market_state_collection: &Collection<CurrentMarketState>){
         let current_market_place = match time_frame {
            TimeFrame::OneMinute => {
                Self::calculate_market_state_for_oneminute(stock)
            },
            TimeFrame::ThreeMinutes => {
                 Self::calculate_market_state_for_threeminutes(stock)
            },
            TimeFrame::FiveMinutes => {
                Self::calculate_market_state_for_fiveminutes(stock)
            },
            TimeFrame::FifteenMinutes => {
                Self::calculate_market_state_for_fifteenminutes(stock)
            },
            TimeFrame::OneDay => {
                Self::calculate_market_state_for_oneday(stock)
            },
            TimeFrame::OneWeek => {
                Self::calculate_market_state_for_oneweek(stock)
            },
            TimeFrame::OneMonth => {
                Self::calculate_market_state_for_onemonth(stock)
            },
            TimeFrame::OneYear => {
                Self::calculate_market_state_for_oneyear(stock)
            },
        };

        match current_market_place {
            Some(current_market_state) => {
                match current_market_state_collection.insert_one(current_market_state.clone(), None).await {
                    Ok(_) => {
                        println!("Successfully inserted a current_market_place into the collection");
                    },
                    Err(e) => {
                        println!("Error while inserting a current_market_place into the collection: {:?} error {:?}", current_market_state,e);
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
            100.0,
            200.0,
            50.0,
            150.0,
            2000.0,
            200.0,
            300.0,
            150.0,
            250.0,
            3000.0,
            2,
            2,
            "ADANIGREEN".to_owned(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            date_parser::new_current_date_time_in_desired_stock_datetime_format()
        ))
    }
    fn calculate_market_state_for_threeminutes(stock: &RawStock)->Option<CurrentMarketState>{
        None
    }
    fn calculate_market_state_for_fiveminutes(stock: &RawStock)->Option<CurrentMarketState>{
        None
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
}

