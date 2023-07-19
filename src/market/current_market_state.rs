
#[path = "../trade/mod.rs"] mod trade;
use self::trade::trade::Trade;
pub enum MarketTrend{
    Bullish,
    Bearish,
    Sideways,
    Flat,
}
pub enum CurrentMarketPosition{
    Long,
    Short,
    Flat,
}

pub struct CurrentMarketState {
    pub current_market_position: CurrentMarketPosition,
    pub current_trade: Trade,
    pub trade_history: Vec<Trade>,
    pub trade_taken_on: String,
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

}

impl CurrentMarketState {
    fn new(current_market_position: CurrentMarketPosition, current_trade: Trade, trade_history: Vec<Trade>, trade_taken_on: String, previous_day_market_trend: MarketTrend, current_day_market_trend: MarketTrend, previous_day_open: f32, previous_day_high: f32, previous_day_low: f32, previous_day_close: f32, previous_day_volume: f32, current_day_open: f32, current_day_high: f32, current_day_low: f32, current_day_close: f32, current_day_volume: f32, last_consecutive_green_candle_count: i32, last_consecutive_red_candle_count: i32) -> CurrentMarketState {
        CurrentMarketState {
            current_market_position,
            current_trade,
            trade_history,
            trade_taken_on,
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
        }
    }

    fn update_current_market_position(&mut self, current_market_position: CurrentMarketPosition, updated_trade: Trade) {
        self.current_market_position = current_market_position;
        self.trade_history.push(updated_trade);
    }
}

