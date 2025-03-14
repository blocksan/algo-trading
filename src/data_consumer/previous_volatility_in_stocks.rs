use std::f32::INFINITY;

use crate::common::raw_stock::RawStock;

pub fn previous_drop_in_stock(stocks: &Vec<RawStock>, max_depth_count: usize, threshold_depth_value: f32) -> bool {
    //taking one extra stock to ignore the current inserted stock
    let mut max_open_price = 0.0;
    let mut previous_candle_close_price = 0.0;
    for (index, stock) in stocks.iter().rev().take(max_depth_count+1).enumerate() {
        if index == 0 {
            continue;
        }
        if index == 1{
            previous_candle_close_price = stock.close;
        }
        if stock.open > max_open_price {
            max_open_price = stock.open;
        }
    }
    
    if max_open_price - previous_candle_close_price > threshold_depth_value {
        return true;
    }else{
        return false;
    }
}

pub fn previous_peak_in_stocks(stocks: &Vec<RawStock>, max_depth_count: usize, threshold_depth_value: f32) -> bool {
    //taking one extra stock to ignore the current inserted stock
    let mut min_open_price = INFINITY;
    let mut previous_candle_close_price = 0.0;
    for (index, stock) in stocks.iter().rev().take(max_depth_count+1).enumerate() {
        if index == 0 {
            continue;
        }
        if index == 1{
            previous_candle_close_price = stock.close;
        }
        if min_open_price > stock.open {
            min_open_price = stock.open;
        }
    }
    
    if previous_candle_close_price - min_open_price > threshold_depth_value {
        return true;
    }else{
        return false;
    }
}