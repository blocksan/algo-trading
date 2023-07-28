pub mod algo_hub;
pub mod common;
pub mod data_consumer;
pub mod order_manager;
pub mod trade_watcher;
pub mod user;
use algo_hub::hammer_pattern::{self, HammerCandle};
use common::redis_client::RedisClient;
use common::utils;
use data_consumer::{
    current_market_state::{self, CurrentMarketState},
    data_consumer_via_csv,
};
use futures::StreamExt;
use order_manager::{
    order_dispatcher,
    pnl_state::{self, CurrentPnLState, PnLConfiguration},
    trade_signal_keeper::{self, TradeSignal},
};
use std::{thread, time};
use trade_watcher::trade_watcher::check_for_exit_opportunity;
use user::user as user_module;
extern crate mongodb;
extern crate tokio;
use mongodb::{options::ClientOptions, Client};

use crate::common::{raw_stock::RawStock, date_parser};
use crate::{common::enums::TimeFrame, order_manager::order_dispatcher::Order};

use std::error::Error;
use tokio_tungstenite::connect_async;
use url::Url;



#[tokio::main]
async fn main() {
    const FILE_5MIN_PATH: &str = "datasets_all_intervals_NSE/ADANIGREEN_5minute_data.csv";
    const FILE_1MIN_PATH: &str = "datasets_all_intervals_NSE/ADANIGREEN_minute_data.csv";

//     let rt = Runtime::new().unwrap();
//     rt.block_on(async move {
//         tokio::spawn(async {
//             connect_websocket_oneminute().await
//         });
//         // tokio::spawn(async {
//         //     connect_websocket_oneminute().await
//         // });
//     }
//    );
    let server_urls = vec![
        "ws://localhost:5554", //oneminute socket
        "ws://localhost:5555", //threeminute socket
        "ws://localhost:5556", //fiveminute socket
        "ws://localhost:5557", //fifteenminute socket
    ];
    let tasks = server_urls
        .into_iter()
        .map(|server_url| {
            tokio::spawn(async move {
                if let Err(e) = connect_websocket(server_url).await {
                    eprintln!("Error connecting to {}: {}", server_url, e);
                }
            })
        })
        .collect::<Vec<_>>();


    //START -> Oneminute Socket reading code
    async fn connect_websocket(server_url: &str) -> Result<(), Box<dyn Error>> {

        let (mut ws_stream, _) = connect_async(Url::parse(server_url).unwrap()).await.unwrap();

        println!("Connected to WebSocket server: {}", server_url);

        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(message) => {
                    if message.is_text() {
                        let text = message.to_text().unwrap();
                        println!("Received on {} tick: {}",server_url, text);
                        // Here, you can process the received CSV data or perform any other actions.
                    }
                }
                Err(e) => {
                    eprintln!("Error while receiving message: {:?}", e);
                }
            }
        }

        Ok(())
    }
    //END -> Oneminute Socket reading code
    // connect_websocket_oneminute().await;    
    // connect_websocket_oneminute().await;
    // return;

    let redis_client = RedisClient::get_instance();

    let mut trade_keeper = trade_signal_keeper::TradeSignalsKeeper::new();

    let stock_5_min_data = data_consumer_via_csv::read_5_min_data(FILE_5MIN_PATH).unwrap();

    let mut order_manager = order_dispatcher::OrderManager::new();

    let mut hammer_ledger = hammer_pattern::HammerPatternUtil::new();

    println!("Connecting to the mongodb");
    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";

    let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    //START -> add new User into the database
    let user_collection_name = "users";
    let user_collection = client
        .database(database_name)
        .collection::<user_module::User>(user_collection_name);

    let user = user_module::User::new(
        1,
        "Rahul".to_owned(),
        "rahul@gmail.com".to_owned(),
        "password".to_owned(),
        date_parser::new_current_date_time_in_desired_stock_datetime_format(),
        date_parser::new_current_date_time_in_desired_stock_datetime_format(),
    );
    user_module::User::add_new_user(&user, user_collection.clone()).await;
    //END -> add new User into the database


    //START -> add the pnl_configuration into the database
    let pnl_configuration_collection_name = "pnl_configurations";
    let pnl_configuration_collection = client
        .database(database_name)
        .collection::<PnLConfiguration>(pnl_configuration_collection_name);

    pnl_state::PnLConfiguration::new_static_config(pnl_configuration_collection.clone()).await;

    //END -> add the pnl_configuration into the database

    //START -> add the current_pnl_state into the database
    let current_pnl_state_collection_name = "current_pnl_states";
    let current_pnl_state_collection = client
        .database(database_name)
        .collection::<CurrentPnLState>(current_pnl_state_collection_name);
    pnl_state::CurrentPnLState::new_static_current_pnl_state(
        current_pnl_state_collection.clone(),
        pnl_configuration_collection.clone(),
    )
    .await;
    //END -> add the current_pnl_state into the database

    //START -> add the current_market_state into the database
    let current_market_state_collection_name = "current_market_states";
    let current_market_state_collection = client
        .database(database_name)
        .collection::<CurrentMarketState>(current_market_state_collection_name);

    let temp_stock = RawStock::new(
        "ADANIGREEN".to_owned(),
        "2021-01-01".to_string(),
        402.5,
        403.3,
        401.6,
        403.3,
        100000.0,
    );

    current_market_state::CurrentMarketState::calculate_market_state(
        &temp_stock,
        TimeFrame::OneMinute,
        &current_market_state_collection,
    )
    .await;

    //END -> add the current_market_state into the database
    futures::future::join_all(tasks).await;

    return;

    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    let db = client.database(database_name);

    // let db = database::get_database(database_name).await;
    let orders_collection_name = "orders";
    let orders_collection = db.collection::<Order>(orders_collection_name);

    let hammer_candle_collection_name = "hammer_candles";
    let hammer_candle_collection = db.collection::<HammerCandle>(hammer_candle_collection_name);

    let trade_signal_collection_name = "trade_signals";
    let trade_signal_collection = db.collection::<TradeSignal>(trade_signal_collection_name);

    for stock in stock_5_min_data.iter() {
        thread::sleep(time::Duration::from_secs(0));

        CurrentMarketState::calculate_market_state(
            stock,
            TimeFrame::FiveMinutes,
            &current_market_state_collection,
        )
        .await;
        hammer_ledger
            .calculate_and_add_ledger(stock, hammer_candle_collection.clone())
            .await; //TODO:: add to database too

        if hammer_ledger.fetch_hammer_pattern_ledger().len() > 0 {
            break;
        }
    }

    match hammer_ledger.check_for_trade_opportunity() {
        Some(trade_signal) => {
            trade_keeper
                .add_trade_signal(&trade_signal, trade_signal_collection)
                .await; //TODO:: add to database too

            match order_manager
                .add_and_dispatch_order(trade_signal, orders_collection)
                .await
            {
                //TODO:: add to database too
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
    // println!("Order Manager => {:?}", order_manager.get_orders());

    let stock_1_min_data = data_consumer_via_csv::read_1_min_data(FILE_1MIN_PATH).unwrap();
    for stock in stock_1_min_data.iter() {
        thread::sleep(time::Duration::from_secs(0));
        // hammer_ledger.calculate_and_add_ledger(stock);
        check_for_exit_opportunity(&mut order_manager, stock.clone()); //TODO:: update to database too
    }
    println!("Order Manager => {:?}", order_manager.get_orders());

    // let collection = db.collection::<bson::Document>(collection_name);

    // let filter = doc! {};

    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // let find_options = mongodb::options::FindOptions::builder()
    // .limit(1)
    //     .build();

    // let mut cursor = collection.find(filter, find_options).await.unwrap();

    // while let Some(result) = cursor.try_next().await.unwrap() {
    // result.to_string();
    // match result {
    //     Ok(doc) => {
    //         // Convert the BSON document to a formatted JSON string
    //         let json_string = doc.to_string_pretty()?;
    //         println!("Result: {}", json_string);
    //         doc
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {:?}", e);
    //     }
    // }
    // let json_string = serde_json::to_string_pretty(&result).unwrap();
    // println!("document => {:?}",json_string);

    // let doc: CompanyWallets = bson::from_document(result).unwrap();
    // println!("deserialized doc =>{:?}", doc);
    //     for (index, data) in result.into_iter().enumerate() {
    //         println!("Data key {:?} value {:?}", data.0, data.1);

    //     }
    // }

    // Ok(())
}
