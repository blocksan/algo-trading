pub mod algo_hub;
pub mod common;
pub mod config;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;

use algo_hub::hammer_pattern;
use common::redis_client::RedisClient;
use data_consumer::current_market_state::CurrentMarketState;
use futures::StreamExt;
use order_manager::{
    order_dispatcher,
    pnl_state::{self, CurrentPnLState, PnLConfiguration},
    trade_signal_keeper,
};
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::Mutex;
extern crate mongodb;
extern crate tokio;
use mongodb::{options::ClientOptions, Client};

use crate::{
    algo_hub::algo_dispatcher,
    common::{
        date_parser,
        enums::{AlgoTypes, RootSystemConfig, ThreadJobType, ThreadWorkerConfig},
        raw_stock::{RawStock, RawStockLedger},
    },
    trade_watcher::monitor_trade,
};
use crate::{common::enums::TimeFrame, order_manager::order_dispatcher::Order};

use dotenv;
use lazy_static::lazy_static;
use std::env;
use std::error::Error;
use std::time::Instant;
use tokio_tungstenite::connect_async;
use url::Url;
use crate::pnl_state::CurrentPnLStateBodyParams;

lazy_static! {
    static ref HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE: f32 = {
        let temp = env::var("HAMMER_LOWER_WICK_HORIZONTAL_SUPPORT_TOLERANCE")
            .unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };
    static ref HAMMER_RED_CANDLES_COUNT_THRESHOLD: i32 = {
        let temp =
            env::var("HAMMER_RED_CANDLES_COUNT_THRESHOLD").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<i32>().unwrap()
    };
    static ref HAMMER_MAX_DROP_THRESHOLD_VALUE: f32 = {
        let temp =
            env::var("HAMMER_MAX_DROP_THRESHOLD_VALUE").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };
    static ref HAMMER_MAX_DROP_CANDLE_COUNT: usize = {
        let temp = env::var("HAMMER_MAX_DROP_CANDLE_COUNT").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<usize>().unwrap()
    };
    static ref HAMMER_SL_MARGIN_POINTS: f32 = {
        let temp = env::var("HAMMER_SL_MARGIN_POINTS").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };
    static ref HAMMER_TARGET_MARGIN_MULTIPLIER: f32 = {
        let temp =
            env::var("HAMMER_TARGET_MARGIN_MULTIPLIER").unwrap_or_else(|_| String::from("0.0"));
        temp.parse::<f32>().unwrap()
    };
}

#[tokio::main]
pub async fn main() {
    let start_time = Instant::now();
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| String::from("development"));
    match environment.as_str() {
        "production" => {
            dotenv::from_filename(".env").ok();
            println!("Using production environment variables");
        }
        _ => {
            dotenv::from_filename(".env.dev").ok();
            println!("Using development environment variables");
        }
    }
    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";

    let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    // let stock_5_min_data = data_consumer_via_csv::read_5_min_data(FILE_5MIN_PATH).unwrap();
    // let hammer_candle_collection_name = "hammer_candles";

    let hammer_ledger = hammer_pattern::HammerPatternUtil::new();

    //START -> add the current_market_state into the database
    // let current_market_state_collection_name = "current_market_states";

    // let orders_collection_name = "orders";

    // let trade_signal_collection_name = "trade_signals";
    let trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();

    let order_manager = order_dispatcher::OrderManager::new();
    let redis_client = RedisClient::get_instance();

    let shared_order_ledger: Arc<Mutex<Vec<Order>>> = Arc::new(Mutex::new(Vec::new()));

    //START -> add new User into the database
    // let user_collection_name = "users";

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

    
    //END -> add the pnl_configuration into the database

    //START -> add the current_pnl_state into the database
    // let current_pnl_state_collection_name = "current_pnl_states";

    //TODO: Only update current market state at 1 min data tick and other time frame can copy the stats from it.
    // 1. current_day_open high close low -> can be picked from 1 min only
    // 2. previous_day_open high close low -> can be picked from 1 min only
    // Others stats will be calculated seperately for each timeframe

    // pnl_state::CurrentPnLState::new_static_current_pnl_state(
    //     current_pnl_state_collection.clone(),
    //     pnl_configuration_collection.clone(),
    // )
    // .await;
    //END -> add the current_pnl_state into the database
    let thread_worker_configs = vec![
        // ThreadWorkerConfig {
        //     thread_job_type: ThreadJobType::DataConsumerViaSocket,
        //     time_frame: TimeFrame::OneMinute,
        //     root_system_config: RootSystemConfig {
        //         database_instance: db.clone(),
        //         hammer_candle_collection: hammer_candle_collection.clone(),
        //         hammer_ledger: hammer_ledger.clone(),
        //         current_market_state_collection: current_market_state_collection.clone(),
        //         orders_collection: orders_collection.clone(),
        //         trade_signal_collection: trade_signal_collection.clone(),
        //         user_collection: user_collection.clone(),
        //         server_url: "ws://localhost:5554".to_string(),
        //         tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
        //         trade_keeper: trade_keeper.clone(),
        //         order_manager: order_manager.clone(),
        //         shared_order_ledger: shared_order_ledger.clone(),
        //         current_pnl_state_collection: current_pnl_state_collection.clone(),
        //         pnl_configuration_collection: pnl_configuration_collection.clone(),
        //     },
        // }, //oneminute socket
        // ThreadWorkerConfig{
        //     server_url: "ws://localhost:5555".to_string(),
        //     time_frame: TimeFrame::ThreeMinutes
        // }, //threeminute socket
        ThreadWorkerConfig {
            thread_job_type: ThreadJobType::DataConsumerViaSocket,
            time_frame: TimeFrame::FiveMinutes,
            root_system_config: RootSystemConfig {
                hammer_ledger: hammer_ledger.clone(),
                server_url: "ws://localhost:5556".to_string(),
                tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
                trade_keeper: trade_keeper.clone(),
                order_manager: order_manager.clone(),
                shared_order_ledger: shared_order_ledger.clone(),
            },
        }, //fiveminute socket
           // ThreadWorkerConfig {
           //     thread_job_type: ThreadJobType::TradeWatcherCron,
           //     time_frame: TimeFrame::Infinity,
           //     root_system_config: RootSystemConfig {
           //         database_instance: db.clone(),
           //         hammer_candle_collection: hammer_candle_collection.clone(),
           //         hammer_ledger: hammer_ledger.clone(),
           //         current_market_state_collection: current_market_state_collection.clone(),
           //         orders_collection: orders_collection.clone(),
           //         trade_signal_collection: trade_signal_collection.clone(),
           //         user_collection: user_collection.clone(),
           //         server_url: "ws://localhost:5557".to_string(),
           //         tradeable_algo_types: vec![AlgoTypes::HammerPatternAlgo],
           //         trade_keeper: trade_keeper.clone(),
           //         order_manager: order_manager.clone(),
           //         shared_order_ledger: shared_order_ledger.clone(),
           //         current_pnl_state_collection: current_pnl_state_collection.clone(),
           //         pnl_configuration_collection: pnl_configuration_collection.clone(),
           //     },
           // }, // "ws://localhost:5556", //fiveminute socket
           // "ws://localhost:5557", //fifteenminute socket
    ];

    let tasks = thread_worker_configs
        .into_iter()
        .map(|thread_worker_config| {
            tokio::spawn(async move {
                if let Err(e) =
                    ingest_data_via_stream(thread_worker_config.clone(), redis_client).await
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
        redis_client: &SyncMutex<RedisClient>,
    ) -> Result<(), Box<dyn Error>> {
        if thread_worker_config.thread_job_type == ThreadJobType::TradeWatcherCron {
            Ok(())
        } else {
            let RootSystemConfig {
                hammer_ledger,
                server_url,
                tradeable_algo_types,
                trade_keeper,
                mut order_manager,
                shared_order_ledger,
            } = thread_worker_config.root_system_config;

            let mut raw_stock_ledger = RawStockLedger::new();

           

            let user_id = "64d8febebe3ea57f392c36df"; //TODO: remove this hardcoding and fetch the user_id from the database
            pnl_state::PnLConfiguration::new_static_backtest_config().await;
            let pnl_configurations_option = PnLConfiguration::fetch_current_pnl_configuration(None, Some(user_id.to_string()), None).await;
            

            if pnl_configurations_option.is_none(){
                println!("No PnL Configuration found for user_id: {}", user_id);
                return Ok(());
            }

            let pnl_configurations = pnl_configurations_option.unwrap();

            if pnl_configurations.len() == 0{
                println!("No PnL Configuration found with length 0 for user_id: {}", user_id);
                return Ok(());
            }

            for pnl_configuration in pnl_configurations.iter() {
                for pnl_symbol in pnl_configuration.symbols.iter(){
                    let symbol = Some(pnl_symbol.to_string().clone());

                    let raw_stocks = RawStock::fetch_raw_stocks(symbol.clone().unwrap(), pnl_configuration.start_trade_date.clone(), pnl_configuration.end_trade_date.clone(), pnl_configuration.time_frame.clone()).await;
                    if raw_stocks.is_none(){
                        println!("No Raw Stocks found for user_id: {}", user_id);
                        continue;
                    }

                    println!("raw_stocks: {:?}", raw_stocks.clone().unwrap().len());

                    for raw_stock in raw_stocks.unwrap().into_iter(){
                        {
                            // let text = message.to_text().unwrap();

                            // let current_pnl_state_patams = CurrentPnLStateBodyParams {
                            //     start_trade_date: pnl_configuration.start_trade_date.clone(),
                            //     symbol: symbol.clone(),
                            //     end_trade_date: None,
                            //     pnl_congiguration_id: Some(pnl_configuration.id.to_string().clone()),
                            //     user_id: Some(pnl_configuration.user_id.to_string().clone()),
                            // };

                            // let splitted_text = text.split(",").collect::<Vec<&str>>();
                            let RawStock{symbol, date,
                            open, high, low, close, volume,market_time_frame} = raw_stock.clone();
                            // println!("splitted_text: {:?}",splitted_text);
                            // continue;
                            let date =
                                match date_parser::parse_date_in_stock_format(&date) {
                                    Ok(date) => Some(date),
                                    Err(e) => {
                                        println!("Error while parsing date {:?}", e);
                                        None
                                    }
                                };

                            // let close = match close.parse::<f32>() {
                            //     Ok(close) => Some(close),
                            //     Err(e) => {
                            //         println!("Error while parsing close {:?}", e);
                            //         None
                            //     }
                            // };

                            // let high = match splitted_text[3].parse::<f32>() {
                            //     Ok(high) => Some(high),
                            //     Err(e) => {
                            //         println!("Error while parsing high {:?}", e);
                            //         None
                            //     }
                            // };

                            // let low = match splitted_text[4].parse::<f32>() {
                            //     Ok(low) => Some(low),
                            //     Err(e) => {
                            //         println!("Error while parsing low {:?}", e);
                            //         None
                            //     }
                            // };

                            // let open = match splitted_text[5].parse::<f32>() {
                            //     Ok(open) => Some(open),
                            //     Err(e) => {
                            //         println!("Error while parsing open {:?}", e);
                            //         None
                            //     }
                            // };
                            //removing "\"" from the end of the string to parse the volume correctly => &splitted_text[6][0..splitted_text[6].len()-1]
                            // let volume = match &splitted_text[6][0..splitted_text[6].len() - 1]
                            //     .parse::<i32>()
                            // {
                            //     Ok(volume) => Some(*volume),
                            //     Err(e) => {
                            //         println!("Error while parsing volume {:?}", e);
                            //         None
                            //     }
                            // };

                            // if date.is_none()
                            //     || close.is_none()
                            //     || high.is_none()
                            //     || low.is_none()
                            //     || open.is_none()
                            //     || volume.is_none()
                            // {
                            //     println!("Header or Some of the values are None");
                            //     continue;
                            // }

                            let raw_stock = RawStock::new(
                                symbol.to_owned(),
                                date.unwrap(),
                                close,
                                high,
                                low,
                                open,
                                volume,
                                market_time_frame,
                            );

                            // println!("Received on {} tick: {:?}",thread_worker_config.time_frame, text);

                            raw_stock_ledger.add_raw_stock(raw_stock.clone());

                            // let temp = shared_order_ledger.lock().unwrap();
                            // temp.push(value)
                            let mut locked_shared_order_ledger = shared_order_ledger.lock().await;

                            match thread_worker_config.time_frame {
                                TimeFrame::FiveMinutes => {
                                    CurrentMarketState::calculate_market_state(
                                        &raw_stock,
                                        raw_stock.market_time_frame.clone(),
                                        redis_client.clone(),
                                        &raw_stock_ledger,
                                    )
                                    .await;
// let current_pnl_state_patams = CurrentPnLStateBodyParams {
                            //     start_trade_date: pnl_configuration.start_trade_date.clone(),
                            //     symbol: symbol.clone(),
                            //     end_trade_date: None,
                            //     pnl_congiguration_id: Some(pnl_configuration.id.to_string().clone()),
                            //     user_id: Some(pnl_configuration.user_id.to_string().clone()),
                            // };
                                    algo_dispatcher::ingest_raw_stock_data(
                                        &raw_stock,
                                        tradeable_algo_types.clone(),
                                        hammer_ledger.clone(),
                                        trade_keeper.clone(),
                                        order_manager.clone(),
                                        redis_client.clone(),
                                        Some(pnl_configuration.id.to_string()),
                                        &mut locked_shared_order_ledger,
                                    )
                                    .await;
                                    monitor_trade::check_for_exit_opportunity(
                                        &mut order_manager,
                                        raw_stock.clone(),
                                        redis_client.clone(),
                                        &mut locked_shared_order_ledger,
                                    )
                                    .await;
                                    monitor_trade::check_for_execute_opportunity(
                                        &mut order_manager,
                                        raw_stock.clone(),
                                        redis_client.clone(),
                                        &mut locked_shared_order_ledger,
                                    )
                                    .await;
                                    drop(locked_shared_order_ledger);
                                }
                                TimeFrame::OneMinute => {
                                    // println!();
                                    // println!("Received Stock: {:?} at Timeframe {}", message.to_text().unwrap(), thread_worker_config.time_frame );
                                    // println!();

                                    // monitor_trade::check_for_execute_opportunity(
                                    //     &mut order_manager,
                                    //     raw_stock.clone(),
                                    //     redis_client.clone(),
                                    //     &mut locked_shared_order_ledger,
                                    // )
                                    // .await;
                                    // let temp = shared_order_ledger.lock().unwrap().clone();
                                    // println!("Shared Data => {:?}", shared_order_ledger.lock().unwrap());
                                    // drop(locked_shared_order_ledger);
                                }
                                _ => (),
                            }

                            // if hammer_ledger.fextch_hammer_pattern_ledger().len() > 0 {
                            //     break;
                            // }
                        }
                    }
                    

                    
                    // let temp = current_pnl_state_patams.clone();
                    //Info: creation of the current_pnl_states will happen while fetching the current_pnl_state for that stock
                    // let mut current_pnl_states = CurrentPnLState::fetch_current_pnl_state(current_pnl_state_patams.clone(), false).await;
                    // if current_pnl_states.is_none(){
                    //     println!("No Current PnL State found for user_id: {}", user_id);

                    //     CurrentPnLState::new_static_current_pnl_state(symbol.unwrap().as_str(), pnl_configuration.id.to_string().as_str(), pnl_configuration.start_trade_date.as_str(), pnl_configuration.end_trade_date.as_str()).await;
                    //     current_pnl_states = CurrentPnLState::fetch_current_pnl_state(current_pnl_state_patams, false).await;
                    //     if current_pnl_states.is_none(){
                    //         println!("Still No Current PnL State found for user_id: {}", user_id);
                    //         continue;
                    //     }
                    // }

                    // for current_pnl_state in current_pnl_states.unwrap().iter(){
                        

                        // for raw_stock in raw_stocks.unwrap().into_iter(){
                        //     println!("******START********");
                        //     println!("raw_stock: {:?}", raw_stock);
                        //     println!("******END*******");
                        // }
                        //TODO: iterate over the raw_stocks and calculate the everything with corresponding current_pnl_state_id which is the id of the current_pnl_state and act as session_id
                    // }
                }

            }
            Ok(())
        }
    }
    //END -> Oneminute Socket reading code
    // return;
    futures::future::join_all(tasks).await;
    let end_time = Instant::now();
    println!(
        "Total Time Taken: {:?}",
        end_time.duration_since(start_time)
    );
    return;

    // println!("Hammer Pattern => {:?}", hammer_ledger.fetch_hammer_pattern_ledger());
    // println!("Trade Signal => {:?}", trade_keeper.get_trade_signals());
    // println!("Order Manager => {:?}", order_manager.get_orders());

    // let stock_1_min_data = data_consumer_via_csv::read_1_min_data(FILE_1MIN_PATH).unwrap();
    // for stock in stock_1_min_data.iter() {
    //     thread::sleep(time::Duration::from_secs(0));
    //     // hammer_ledger.calculate_and_add_ledger(stock);
    //     check_for_execute_opportunity(&mut order_manager, stock.clone()); //TODO:: update to database too
    // }
    // println!("Order Manager => {:?}", order_manager.get_orders());

    // Ok(())
}
