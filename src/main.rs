pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
use algo_hub::hammer_pattern;
use common::redis_client::RedisClient;
use common::utils;
use data_consumer::data_consumer_via_csv;
use order_manager::{order_dispatcher, trade_signal_keeper};
use std::{thread, time};
use trade_watcher::trade_watcher::check_for_exit_opportunity;

#[allow(dead_code)]
fn main() {
    const FILE_5MIN_PATH: &str = "datasets_all_intervals_NSE/ADANIGREEN_5minute_data.csv";
    const FILE_1MIN_PATH: &str = "datasets_all_intervals_NSE/ADANIGREEN_minute_data.csv";

    let redis_client = RedisClient::get_instance();

    let mut trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();

    let stock_5_min_data = data_consumer_via_csv::read_5_min_data(FILE_5MIN_PATH).unwrap();

    let mut order_manager = order_dispatcher::OrderManager::new();

    let mut hammer_ledger = hammer_pattern::HammerPatternUtil::new();
    for stock in stock_5_min_data.iter() {
        thread::sleep(time::Duration::from_secs(0));
        hammer_ledger.calculate_and_add_ledger(stock);

        if hammer_ledger.fetch_hammer_pattern_ledger().len() > 0 {
            break;
        }
        // println!("Hammer Pattern => {:?}", hammer_ledger.fetch_hammer_pattern_ledger());
    }

    match hammer_ledger.check_for_trade_opportunity() {
        Some(trade_signal) => {
            trade_keeper.add_trade_signal(trade_signal.clone());

            match order_manager.add_and_dispatch_order(trade_signal) {
                Some(order) => {
                    let key = utils::symbol_algo_type_formatter(
                        order.symbol.as_str(),
                        order.trade_algo_type.to_string().as_str(),
                    );

                    match redis_client.lock().unwrap().set_data(key.as_str(), 1) {
                        Ok(_) => {
                            println!("Data set in Redis for key => {}", key);
                        }
                        Err(e) => {
                            println!("Error while setting the data in Redis => {:?}", e);
                        }
                    }
                    println!("Order Placed => {:?}", order);
                }
                None => {
                    println!("Order not placed");
                }
            }
        }
        None => {
            println!("No Trade Opportunity Found");
        }
    }


    // println!("Hammer Pattern => {:?}", hammer_ledger.fetch_hammer_pattern_ledger());
    // println!("Trade Signal => {:?}", trade_keeper.get_trade_signals());
    println!("Order Manager => {:?}", order_manager.get_orders());

    let stock_1_min_data = data_consumer_via_csv::read_1_min_data(FILE_1MIN_PATH).unwrap();
    for stock in stock_1_min_data.iter(){
        thread::sleep(time::Duration::from_secs(0));
        // hammer_ledger.calculate_and_add_ledger(stock);
        check_for_exit_opportunity(&mut order_manager, stock.clone());
    }
    println!("Order Manager => {:?}", order_manager.get_orders());
}
