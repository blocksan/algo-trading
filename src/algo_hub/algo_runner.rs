
use crate::{
    algo_hub::{algo_dispatcher, hammer_pattern},
    common::{
        date_parser,
        enums::{AlgoTypes, RootSystemConfig, ThreadJobType, ThreadWorkerConfig},
        raw_stock::{RawStock, RawStockLedger}, redis_client::RedisClient,
    },
    trade_watcher::monitor_trade, order_manager::{trade_signal_keeper, order_dispatcher, pnl_state::{ PnLConfiguration, STATIC_USER_ID, CurrentPnLState, CurrentPnLStateBodyParams}}, data_consumer::current_market_state::CurrentMarketState,
};
use crate::{common::enums::TimeFrame, order_manager::order_dispatcher::Order};

use dotenv;
use lazy_static::lazy_static;
use std::{error::Error, env};
use std::time::Instant;
use std::sync::{Arc, Mutex as SyncMutex};
use tokio::sync::Mutex;

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

// #[tokio::main]
pub async fn backtest_strategy(pnl_configuration_id: String) {
    // let pnl_configuration_id = "6500bda2b5fbcf0f36cf3f7e";

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

    let hammer_ledger = hammer_pattern::HammerPatternUtil::new();
    
    let trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();

    let order_manager = order_dispatcher::OrderManager::new();
    let redis_client = RedisClient::get_instance();

    let shared_order_ledger: Arc<Mutex<Vec<Order>>> = Arc::new(Mutex::new(Vec::new()));

    let thread_worker_configs = vec![
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
                pnl_configuration_id
            },
        }, //fiveminute socket
           
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

    async fn ingest_data_via_stream(
        thread_worker_config: ThreadWorkerConfig,
        redis_client: &SyncMutex<RedisClient>,
    ) -> Result<(), Box<dyn Error>> {
        if thread_worker_config.thread_job_type == ThreadJobType::TradeWatcherCron {
            return Ok(())
        } else {
            let RootSystemConfig {
                hammer_ledger,
                server_url:_,
                tradeable_algo_types,
                trade_keeper,
                mut order_manager,
                shared_order_ledger,
                pnl_configuration_id
            } = thread_worker_config.root_system_config;

            let mut raw_stock_ledger = RawStockLedger::new();

           

            // PnLConfiguration::new_static_backtest_config().await;
            // return Ok(());
            let current_pnl_state_params = CurrentPnLStateBodyParams{
                start_trade_date: None,
                symbol: None,
                end_trade_date: None,
                pnl_congiguration_id: Some(pnl_configuration_id.clone()),
                user_id: None
            };
            let current_pnl_states_option = CurrentPnLState::fetch_current_pnl_state(current_pnl_state_params, false).await;
            

            if current_pnl_states_option.is_none(){
                println!("No Current PnL State found for pnl_configuration_id: {}", pnl_configuration_id);
                return Ok(());
            }

            let current_pnl_states = current_pnl_states_option.unwrap();

            if current_pnl_states.len() == 0{
                println!("No Current PnL State found with length 0 for pnl_configuration_id: {}", pnl_configuration_id);
                return Ok(());
            }

            for current_pnl_state in current_pnl_states.iter() {

                    let raw_stocks = RawStock::fetch_raw_stocks(current_pnl_state.symbol.clone(), current_pnl_state.start_trade_date.clone(), current_pnl_state.end_trade_date.clone(), current_pnl_state.time_frame.clone()).await;
                    if raw_stocks.is_none(){
                        println!("No Raw Stocks found for date range: {:?} {:?}", current_pnl_state.start_trade_date.clone(), current_pnl_state.end_trade_date.clone());
                        continue;
                    }

                    println!("raw_stocks: {:?}", raw_stocks.clone().unwrap().len());

                    for raw_stock in raw_stocks.unwrap().into_iter(){
                        {
                            // let current_pnl_state_patams = CurrentPnLStateBodyParams {
                            //     start_trade_date: pnl_configuration.start_trade_date.clone(),
                            //     symbol: symbol.clone(),
                            //     end_trade_date: None,
                            //     pnl_congiguration_id: Some(pnl_configuration.id.to_string().clone()),
                            //     user_id: Some(pnl_configuration.user_id.to_string().clone()),
                            // };

                            let RawStock{symbol, date,
                            open, high, low, close, volume,market_time_frame} = raw_stock.clone();
                            let date =
                                match date_parser::parse_date_in_stock_format(&date) {
                                    Ok(date) => Some(date),
                                    Err(e) => {
                                        println!("Error while parsing date {:?}", e);
                                        None
                                    }
                                };

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


                            raw_stock_ledger.add_raw_stock(raw_stock.clone());

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
                                    algo_dispatcher::ingest_raw_stock_data(
                                        &raw_stock,
                                        tradeable_algo_types.clone(),
                                        hammer_ledger.clone(),
                                        trade_keeper.clone(),
                                        order_manager.clone(),
                                        redis_client.clone(),
                                        Some(current_pnl_state.pnl_configuration_id.to_string()),
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
                                _ => (),
                            }
                        }
                    }
            }
        }
            Ok(())
        
    }
    // return;
    futures::future::join_all(tasks).await;
    let end_time = Instant::now();
    println!(
        "Total Time Taken: {:?}",
        end_time.duration_since(start_time)
    );
    return;
}


pub async fn create_static_pnl_config() -> Option<Vec<PnLConfiguration>>{
    PnLConfiguration::new_static_backtest_config().await;
    let pnl_configuration = PnLConfiguration::fetch_current_pnl_configuration(None, Some(STATIC_USER_ID.to_string()), None).await;
    pnl_configuration
}

pub async fn create_curren_pnl_states(pnl_configurations: Option<Vec<PnLConfiguration>>) -> Option<String>{
    CurrentPnLState::create_current_pnl_states(pnl_configurations).await
}