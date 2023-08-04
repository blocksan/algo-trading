use crate::common::enums::{TimeFrame, AlgoTypes};

//ORDER_TradeAlgoType_Symbol
pub fn order_cache_key_formatter(symbol: &str, algo_type: &AlgoTypes) -> String {
    format!("ORDER_{}_{}", symbol, algo_type.to_string())
}

pub fn current_market_state_cache_key_formatter(trade_date_only: &str, symbol: &str, market_time_frame: &TimeFrame) -> String {
    format!("{}_{}_{}_{}","CMS" , trade_date_only, symbol, market_time_frame)
}