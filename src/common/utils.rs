use crate::common::enums::{TimeFrame, AlgoTypes};

//ORDER_TradeAlgoType_Symbol_Date
pub fn order_cache_key_formatter(symbol: &str, algo_type: &AlgoTypes, date: &str) -> String {
    //want to fetch the numbers from the date 2025-03-13 01:12:13+05:30
    let first_date_split = date.split(" ").collect::<Vec<&str>>();
    let date_parts_2 = first_date_split[0].split("-").collect::<Vec<&str>>();
    let year = date_parts_2[0];
    let month = date_parts_2[1];
    let day = date_parts_2[2];
    let time_parts = first_date_split[1].split(":").collect::<Vec<&str>>();
    let hour = time_parts[0];
    let minute = time_parts[1];
    let second = time_parts[2];

    format!("ORDER_{}_{}_{}_{}_{}_{}_{}_{}", symbol, algo_type.to_string(), year, month, day, hour, minute, second)
}
pub fn current_market_state_cache_key_formatter(trade_date_only: &str, symbol: &str, market_time_frame: &TimeFrame) -> String {
    format!("{}_{}_{}_{}","CMS" , trade_date_only, symbol, market_time_frame)
}