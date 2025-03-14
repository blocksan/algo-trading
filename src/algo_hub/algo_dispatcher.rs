use std::sync::Mutex as SyncMutex;

use crate::{common::{raw_stock::RawStock, enums::AlgoTypes, redis_client::RedisClient}, 
order_manager::{trade_signal_keeper::TradeSignalsKeeper, order_dispatcher::{Order, OrderManager}}};

use super::hammer_pattern::HammerPatternUtil;

pub async fn ingest_raw_stock_data(
    raw_stock: &RawStock, 
    tradeable_algo_types: Vec<AlgoTypes>, 
    mut hammer_ledger: HammerPatternUtil, 
    mut trade_keeper: TradeSignalsKeeper, 
    mut order_manager: OrderManager,
    redis_client: &SyncMutex<RedisClient>,
    shared_order_ledger: &mut Vec<Order>,
){

    // mut hammer_ledger: HammerPatternUtil, hammer_candle_collection: Collection<HammerCandle>
    //TODO: this is just to consume the data
    // I do need to make it more configurable using threads here, so that I can run multiple algorithms at the same time
    println!("ingest_raw_stock_data");
    for tradeable_algo_type in tradeable_algo_types.iter() {
        match tradeable_algo_type {
            AlgoTypes::HammerPatternAlgo => {
                let trade_signal_option = hammer_ledger
                .calculate_and_add_ledger(&raw_stock, None)
                .await; 
                // println!("Trade Signal Option: {:?}", trade_signal_option);
                match trade_signal_option {
                    Some(trade_signal) => {
                        trade_keeper
                            .add_trade_signal(&trade_signal)
                            .await;
                        order_manager.check_and_dispatch_order(trade_signal,redis_client, shared_order_ledger).await;
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


pub async fn backtest_ingest_raw_stock_data(
    raw_stock: &RawStock, 
    tradeable_algo_types: Vec<AlgoTypes>, 
    mut hammer_ledger: HammerPatternUtil, 
    mut trade_keeper: TradeSignalsKeeper, 
    mut order_manager: OrderManager,
    redis_client: &SyncMutex<RedisClient>,
    current_pnl_state_id: String,
    shared_order_ledger: &mut Vec<Order>,
){
    for tradeable_algo_type in tradeable_algo_types.iter() {
        match tradeable_algo_type {
            AlgoTypes::HammerPatternAlgo => {
                let trade_signal_option = hammer_ledger
                .calculate_and_add_ledger(&raw_stock, Some(current_pnl_state_id.clone()))
                .await; 
                // println!("Trade Signal Option: {:?}", trade_signal_option);
                match trade_signal_option {
                    Some(trade_signal) => {
                        trade_keeper
                            .add_trade_signal(&trade_signal)
                            .await;
                        order_manager.backtest_check_and_dispatch_order(trade_signal,redis_client, shared_order_ledger).await;
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