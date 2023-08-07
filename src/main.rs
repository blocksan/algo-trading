pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;
use algo_hub::hammer_pattern::{self, HammerCandle};
use common::redis_client::RedisClient;
use data_consumer::current_market_state::CurrentMarketState;
use futures::StreamExt;
use order_manager::{
    order_dispatcher,
    pnl_state::{self, CurrentPnLState, PnLConfiguration},
    trade_signal_keeper::{self, TradeSignal},
};
use std::sync::{Mutex, Arc};
extern crate mongodb;
extern crate tokio;
use mongodb::{options::ClientOptions, Client};

use crate::{
    algo_hub::algo_dispatcher,
    common::{
        date_parser,
        enums::{AlgoTypes, RootSystemConfig, ThreadJobType, ThreadWorkerConfig},
        raw_stock::{RawStock, RawStockLedger},
    }, trade_watcher::monitor_trade,
};
use crate::{common::enums::TimeFrame, order_manager::order_dispatcher::Order};

use std::error::Error;
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() {
    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";

    let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(database_name);

    // let stock_5_min_data = data_consumer_via_csv::read_5_min_data(FILE_5MIN_PATH).unwrap();
    let hammer_candle_collection_name = "hammer_candles";
    let hammer_candle_collection = db.collection::<HammerCandle>(hammer_candle_collection_name);

    let hammer_ledger = hammer_pattern::HammerPatternUtil::new();

    //START -> add the current_market_state into the database
    let current_market_state_collection_name = "current_market_states";
    let current_market_state_collection = client
        .database(database_name)
        .collection::<CurrentMarketState>(current_market_state_collection_name);

    let orders_collection_name = "orders";
    let orders_collection = db.collection::<Order>(orders_collection_name);

    let trade_signal_collection_name = "trade_signals";
    let trade_signal_collection = db.collection::<TradeSignal>(trade_signal_collection_name);
    let trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();

    let order_manager = order_dispatcher::OrderManager::new();
    let redis_client = RedisClient::get_instance();

    let shared_order_ledger: Arc<Mutex<Vec<Order>>> = Arc::new(Mutex::new(Vec::new()));

    //START -> add new User into the database
    // let user_collection_name = "users";
    // let user_collection = client
    //     .database(database_name)
    //     .collection::<user_module::User>(user_collection_name);

    // let user = user_module::User::new(
    //     1,
    //     "Rahul".to_owned(),
    //     "rahul@gmail.com".to_owned(),
    //     "password".to_owned(),
    //     date_parser::new_current_date_time_in_desired_stock_datetime_format(),
    //     date_parser::new_current_date_time_in_desired_stock_datetime_format(),
    // );
    // user_module::User::add_new_user(&user, user_collection.clone()).await;
    //END -> add new User into the database

    //START -> add the pnl_configuration into the database
    // let pnl_configuration_collection_name = "pnl_configurations";
    // let pnl_configuration_collection = client
    //     .database(database_name)
    //     .collection::<PnLConfiguration>(pnl_configuration_collection_name);

    // pnl_state::PnLConfiguration::new_static_config(pnl_configuration_collection.clone()).await;
    //END -> add the pnl_configuration into the database

    //START -> add the current_pnl_state into the database
    // let current_pnl_state_collection_name = "current_pnl_states";

    //TODO: Only update current market state at 1 min data tick and other time frame can copy the stats from it.
    // 1. current_day_open high close low -> can be picked from 1 min only
    // 2. previous_day_open high close low -> can be picked from 1 min only
    // Others stats will be calculated seperately for each timeframe

    // let current_pnl_state_collection = client
    //     .database(database_name)
    //     .collection::<CurrentPnLState>(current_pnl_state_collection_name);

    // pnl_state::CurrentPnLState::new_static_current_pnl_state(
    //     current_pnl_state_collection.clone(),
    //     pnl_configuration_collection.clone(),
    // )
    // .await;
    //END -> add the current_pnl_state into the database

    let thread_worker_configs = vec![
        ThreadWorkerConfig {
            thread_job_type: ThreadJobType::DataConsumerViaSocket,
            time_frame: TimeFrame::OneMinute,
            root_system_config: RootSystemConfig {
                database_instance: db.clone(),
                hammer_candle_collection: hammer_candle_collection.clone(),
                hammer_ledger: hammer_ledger.clone(),
                current_market_state_collection: current_market_state_collection.clone(),
                orders_collection: orders_collection.clone(),
                trade_signal_collection: trade_signal_collection.clone(),
                server_url: "ws://localhost:5554".to_string(),
                tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
                trade_keeper: trade_keeper.clone(),
                order_manager: order_manager.clone(),
                shared_order_ledger: shared_order_ledger.clone(),
            },
        }, //oneminute socket
        // ThreadWorkerConfig{
        //     server_url: "ws://localhost:5555".to_string(),
        //     time_frame: TimeFrame::ThreeMinutes
        // }, //threeminute socket
        ThreadWorkerConfig {
            thread_job_type: ThreadJobType::DataConsumerViaSocket,
            time_frame: TimeFrame::FiveMinutes,
            root_system_config: RootSystemConfig {
                database_instance: db.clone(),
                hammer_candle_collection: hammer_candle_collection.clone(),
                hammer_ledger: hammer_ledger.clone(),
                current_market_state_collection: current_market_state_collection.clone(),
                orders_collection: orders_collection.clone(),
                trade_signal_collection: trade_signal_collection.clone(),
                server_url: "ws://localhost:5556".to_string(),
                tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
                trade_keeper: trade_keeper.clone(),
                order_manager: order_manager.clone(),
                shared_order_ledger: shared_order_ledger.clone(),
            },
        }, //fiveminute socket
        ThreadWorkerConfig {
            thread_job_type: ThreadJobType::TradeWatcherCron,
            time_frame: TimeFrame::Infinity,
            root_system_config: RootSystemConfig {
                database_instance: db.clone(),
                hammer_candle_collection: hammer_candle_collection.clone(),
                hammer_ledger: hammer_ledger.clone(),
                current_market_state_collection: current_market_state_collection.clone(),
                orders_collection: orders_collection.clone(),
                trade_signal_collection: trade_signal_collection.clone(),
                server_url: "ws://localhost:5557".to_string(),
                tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
                trade_keeper: trade_keeper.clone(),
                order_manager: order_manager.clone(),
                shared_order_ledger: shared_order_ledger.clone(),
            },
        }, // "ws://localhost:5556", //fiveminute socket
           // "ws://localhost:5557", //fifteenminute socket
    ];

    let tasks = thread_worker_configs
        .into_iter()
        .map(|thread_worker_config| {
            tokio::spawn(async move {
                if let Err(e) = ingest_data_via_stream(thread_worker_config.clone(), redis_client).await
                {
                    eprintln!(
                        "Error connecting to {:?}: {}",
                        thread_worker_config.root_system_config.server_url, e
                    );
                }
            })
        })
        .collect::<Vec<_>>();

    //START -> Oneminute Socket reading code
    async fn ingest_data_via_stream(
        thread_worker_config: ThreadWorkerConfig,
        redis_client: &Mutex<RedisClient>,
    ) -> Result<(), Box<dyn Error>> {
        if thread_worker_config.thread_job_type == ThreadJobType::TradeWatcherCron {
            Ok(())
        } else {
            let RootSystemConfig {
                database_instance,
                hammer_candle_collection,
                hammer_ledger,
                current_market_state_collection,
                orders_collection,
                trade_signal_collection,
                server_url,
                tradeable_algo_types,
                trade_keeper,
                mut order_manager,
                shared_order_ledger,
            } = thread_worker_config.root_system_config;

            let mut raw_stock_ledger = RawStockLedger::new();


            let (mut ws_stream, _) = connect_async(Url::parse(&server_url).unwrap())
                .await
                .unwrap();

            println!("Connected to WebSocket server: {}", server_url);

            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(message) => {
                        if message.is_text() {
                            let text = message.to_text().unwrap();

                            let splitted_text = text.split(",").collect::<Vec<&str>>();
                            // println!("splitted_text: {:?}",splitted_text);
                            // continue;
                            let date =
                                match date_parser::parse_date_in_stock_format(splitted_text[1]) {
                                    Ok(date) => Some(date),
                                    Err(e) => {
                                        println!("Error while parsing date {:?}", e);
                                        None
                                    }
                                };

                            let close = match splitted_text[2].parse::<f32>() {
                                Ok(close) => Some(close),
                                Err(e) => {
                                    println!("Error while parsing close {:?}", e);
                                    None
                                }
                            };

                            let high = match splitted_text[3].parse::<f32>() {
                                Ok(high) => Some(high),
                                Err(e) => {
                                    println!("Error while parsing high {:?}", e);
                                    None
                                }
                            };

                            let low = match splitted_text[4].parse::<f32>() {
                                Ok(low) => Some(low),
                                Err(e) => {
                                    println!("Error while parsing low {:?}", e);
                                    None
                                }
                            };

                            let open = match splitted_text[5].parse::<f32>() {
                                Ok(open) => Some(open),
                                Err(e) => {
                                    println!("Error while parsing open {:?}", e);
                                    None
                                }
                            };
                            //removing "\"" from the end of the string to parse the volume correctly => &splitted_text[6][0..splitted_text[6].len()-1]
                            let volume = match &splitted_text[6][0..splitted_text[6].len() - 1]
                                .parse::<i32>()
                            {
                                Ok(volume) => Some(*volume),
                                Err(e) => {
                                    println!("Error while parsing volume {:?}", e);
                                    None
                                }
                            };

                            if date.is_none()
                                || close.is_none()
                                || high.is_none()
                                || low.is_none()
                                || open.is_none()
                                || volume.is_none()
                            {
                                println!("Header or Some of the values are None");
                                continue;
                            }

                            let raw_stock = RawStock::new(
                                "ADANIGREEN".to_owned(),
                                date.unwrap(),
                                close.unwrap(),
                                high.unwrap(),
                                low.unwrap(),
                                open.unwrap(),
                                volume.unwrap(),
                                thread_worker_config.time_frame.clone(),
                            );

                            // println!("Received on {} tick: {:?}",thread_worker_config.time_frame, text);

                            raw_stock_ledger.add_raw_stock(raw_stock.clone());

                            match thread_worker_config.time_frame {
                                TimeFrame::FiveMinutes => {
                                    CurrentMarketState::calculate_market_state(
                                        &raw_stock,
                                        thread_worker_config.time_frame.clone(),
                                        &current_market_state_collection,
                                        redis_client.clone(),
                                        &raw_stock_ledger,
                                        database_instance.clone(),
                                    )
                                    .await;
    
                                algo_dispatcher::ingest_raw_stock_data(
                                    &raw_stock,
                                    tradeable_algo_types.clone(),
                                    hammer_ledger.clone(),
                                    hammer_candle_collection.clone(),
                                    trade_keeper.clone(),
                                    trade_signal_collection.clone(),
                                    order_manager.clone(),
                                    orders_collection.clone(),
                                    redis_client.clone(),
                                    database_instance.clone(),
                                    shared_order_ledger.clone(),
                                )
                                .await;
                                },
                                TimeFrame::OneMinute =>{
                                    monitor_trade::check_for_exit_opportunity(&mut order_manager, raw_stock.clone(), redis_client.clone(), orders_collection.clone(), shared_order_ledger.clone()).await;
                                    // let temp = shared_order_ledger.lock().unwrap().clone();
                                    // println!("Shared Data => {:?}", shared_order_ledger.lock().unwrap());
                                }
                                _ => ()
                            }

                            
                            // if hammer_ledger.fextch_hammer_pattern_ledger().len() > 0 {
                            //     break;
                            // }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while receiving message: {:?}", e);
                    }
                }
            }

            Ok(())
        }
    }
    //END -> Oneminute Socket reading code
    // return;
    futures::future::join_all(tasks).await;

    return;

    // println!("Hammer Pattern => {:?}", hammer_ledger.fetch_hammer_pattern_ledger());
    // println!("Trade Signal => {:?}", trade_keeper.get_trade_signals());
    // println!("Order Manager => {:?}", order_manager.get_orders());

    // let stock_1_min_data = data_consumer_via_csv::read_1_min_data(FILE_1MIN_PATH).unwrap();
    // for stock in stock_1_min_data.iter() {
    //     thread::sleep(time::Duration::from_secs(0));
    //     // hammer_ledger.calculate_and_add_ledger(stock);
    //     check_for_exit_opportunity(&mut order_manager, stock.clone()); //TODO:: update to database too
    // }
    // println!("Order Manager => {:?}", order_manager.get_orders());

    // Ok(())
}
