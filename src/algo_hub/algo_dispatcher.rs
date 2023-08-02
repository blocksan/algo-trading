use std::sync::Mutex;
use mongodb::{Collection, Database};

use crate::{common::{raw_stock::RawStock, enums::AlgoTypes, utils, redis_client::RedisClient}, 
order_manager::{self, trade_signal_keeper::{TradeSignal, TradeSignalsKeeper}}};

use super::hammer_pattern::{HammerCandle, HammerPatternUtil};


pub async fn ingest_raw_stock_data(raw_stock: &RawStock, tradeable_algo_types: Vec<AlgoTypes>, 
    mut hammer_ledger: HammerPatternUtil, 
    hammer_candle_collection: Collection<HammerCandle>,
    mut trade_keeper: TradeSignalsKeeper, 
    trade_signal_collection: Collection<TradeSignal>,
    mut order_manager: order_manager::order_dispatcher::OrderManager,
    orders_collection: Collection<order_manager::order_dispatcher::Order>,
    redis_client: &Mutex<RedisClient>,
    _database_instance: Database
){

    // mut hammer_ledger: HammerPatternUtil, hammer_candle_collection: Collection<HammerCandle>
    //TODO: this is just to consume the data
    // I do need to make it more configurable using threads here, so that I can run multiple algorithms at the same time

    for tradeable_algo_type in tradeable_algo_types.iter() {
        match tradeable_algo_type {
            AlgoTypes::HammerPatternAlgo => {
                let trade_signal_option = hammer_ledger
                .calculate_and_add_ledger(&raw_stock, hammer_candle_collection.clone())
                .await; 

                match trade_signal_option {
                    Some(trade_signal) => {
                        trade_keeper
                            .add_trade_signal(&trade_signal, trade_signal_collection.clone())
                            .await; //TODO:: add to database too
            
                        match order_manager
                            .add_and_dispatch_order(trade_signal, orders_collection.clone())
                            .await
                        {
                            Some(order) => {
                                let key = utils::symbol_algo_type_formatter(
                                    order.symbol.as_str(),
                                    order.trade_algo_type.to_string().as_str(),
                                );
            
                                match redis_client.lock().unwrap().set_data(key.as_str(), "1") {
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


            },
            AlgoTypes::ShootingStarPatternAlgo => {

            },
        }
    }
}