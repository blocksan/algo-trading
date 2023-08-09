use std::sync::{Mutex as SyncMutex};
use mongodb::{Collection, Database};

use crate::{common::{raw_stock::RawStock, enums::AlgoTypes, redis_client::RedisClient}, 
order_manager::{self, trade_signal_keeper::{TradeSignal, TradeSignalsKeeper}, order_dispatcher::Order}};

use super::hammer_pattern::{HammerCandle, HammerPatternUtil};


pub async fn ingest_raw_stock_data(raw_stock: &RawStock, tradeable_algo_types: Vec<AlgoTypes>, 
    mut hammer_ledger: HammerPatternUtil, 
    hammer_candle_collection: Collection<HammerCandle>,
    mut trade_keeper: TradeSignalsKeeper, 
    trade_signal_collection: Collection<TradeSignal>,
    mut order_manager: order_manager::order_dispatcher::OrderManager,
    orders_collection: Collection<order_manager::order_dispatcher::Order>,
    redis_client: &SyncMutex<RedisClient>,
    _database_instance: Database,
    shared_order_ledger: &mut Vec<Order>
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
                // println!("Trade Signal Option: {:?}", trade_signal_option);
                match trade_signal_option {
                    Some(trade_signal) => {
                        trade_keeper
                            .add_trade_signal(&trade_signal, trade_signal_collection.clone())
                            .await;
                        order_manager.check_and_dispatch_order(trade_signal,redis_client, orders_collection.clone(), shared_order_ledger).await;
                    }
                    None => {
                        // println!("No Trading Signal Opportunity Found");
                    }
                }


            },
            AlgoTypes::ShootingStarPatternAlgo => {

            },
        }
    }
}