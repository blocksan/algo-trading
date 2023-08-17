use crate::common::enums::{TimeFrame, AlgoTypes};

//ORDER_TradeAlgoType_Symbol
pub fn order_cache_key_formatter(symbol: &str, algo_type: &AlgoTypes, user_id: &str) -> String {
    format!("ORDER_{}_{}_{}", symbol, algo_type.to_string(), user_id)
}

pub fn current_market_state_cache_key_formatter(trade_date_only: &str, symbol: &str, market_time_frame: &TimeFrame) -> String {
    format!("{}_{}_{}_{}","CMS" , trade_date_only, symbol, market_time_frame)
}

pub fn current_pnl_state_cache_key_formatted(trade_date_only: &str, logged_in_user: &str) -> String {
    format!("{}_{}_{}","CPnL" , trade_date_only, logged_in_user)
}

pub fn current_pnl_state_cache_key_algotypes_formatted(trade_date_only: &str, logged_in_user: &str) -> String {
    format!("{}_{}_{}","CPnL_AlgoTypes_" , trade_date_only, logged_in_user)
}