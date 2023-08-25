use std::str::FromStr;

use chrono::DateTime;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOptions, UpdateOptions},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::common::{
    date_parser::{self, new_current_date_time_in_desired_stock_datetime_format},
    enums::AlgoTypes,
    redis_client::RedisClient,
    utils,
};

use super::order_dispatcher::Order;

pub const CURRENT_STOCK_DATE: &str = "2022-10-22 09:40:00+05:30";
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentPnLState {
    pub start_trade_date: String,
    pub end_trade_date: String,
    pub current_pnl: f32,
    pub current_pnl_percentage: f32,
    pub targeted_pnl: f32,
    pub targeted_pnl_percentage: f32,
    pub current_sl_hit_count: i32,
    pub max_sl_hit_count: i32,
    pub current_trade_count: i32,
    pub max_trade_count: i32,
    pub current_target_hit_count: i32,
    pub max_target_hit_count: i32,
    pub target_hit_percentage: f32,
    pub max_sl_capacity: f32,
    pub max_trade_capital: f32,
    pub current_used_trade_capital: f32,
    pub is_eligible_for_trading: bool,
    pub not_eligible_trading_reason: String,
    pub current_pnl_state_cache_key: String,
    pub pnl_configuration_id: ObjectId,
    pub trading_algo_types: Vec<AlgoTypes>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: ObjectId,
}

impl CurrentPnLState {
    pub fn new(
        start_trade_date: String,
        end_trade_date: String,
        current_pnl: f32, //value can be saved in minus if in loss
        current_pnl_percentage: f32,
        targeted_pnl: f32,
        targeted_pnl_percentage: f32,
        current_sl_hit_count: i32,
        max_sl_hit_count: i32,
        current_trade_count: i32,
        max_trade_count: i32,
        current_target_hit_count: i32,
        max_target_hit_count: i32,
        target_hit_percentage: f32,
        max_sl_capacity: f32, //value will be saved in minus
        max_trade_capital: f32,
        is_eligible_for_trading: bool,
        not_eligible_trading_reason: String,
        current_used_trade_capital: f32,
        current_pnl_state_cache_key: String,
        pnl_configuration_id: ObjectId,
        trading_algo_types: Vec<AlgoTypes>,
        id: ObjectId,
        created_at: String,
        updated_at: String,
        user_id: ObjectId,
    ) -> CurrentPnLState {
        CurrentPnLState {
            start_trade_date,
            end_trade_date,
            current_pnl,
            current_pnl_percentage,
            targeted_pnl,
            targeted_pnl_percentage,
            current_sl_hit_count,
            max_sl_hit_count,
            current_trade_count,
            max_trade_count,
            current_target_hit_count,
            max_target_hit_count,
            target_hit_percentage,
            max_sl_capacity,
            max_trade_capital,
            is_eligible_for_trading,
            not_eligible_trading_reason,
            current_used_trade_capital,
            current_pnl_state_cache_key,
            pnl_configuration_id,
            trading_algo_types,
            id,
            created_at,
            updated_at,
            user_id,
        }
    }

    pub async fn new_static_current_pnl_state(
        current_pnl_state_collection: Collection<CurrentPnLState>,
        pnl_configuration_collection: Collection<PnLConfiguration>,
        user_id: String,
        trading_date: String,
    ) {

        let (current_pnl_cache_key, _current_pnl_cache_algo_types_key ) =
            Self::get_current_pnl_cache_key(trading_date.clone().as_str(), user_id.as_str());

        let current_pnl_state_option =
            Self::fetch_current_pnl_state(current_pnl_cache_key.as_str(), true);

        if current_pnl_state_option.is_some() {
            return ();
        }else{


        // let start_dated_formatted = DateTime::parse_from_str(CURRENT_STOCK_DATE, "%Y-%m-%d %H:%M:%S%z").unwrap();
        let filter = doc! {
            "user_id": ObjectId::from_str(user_id.as_str()).unwrap(),
            "is_active": true,
        }; 
            //TODO: fetch it for the current day only by adding end_trade_date as well
           //DateTime::parse_from_str(CURRENT_STOCK_DATE, "%Y-%m-%d %H:%M:%S%z").unwrap().to_string();
        let options = FindOptions::builder().build();

        let mut pnl_configuration_found: Option<Vec<PnLConfiguration>> = None;
        // let cursor = pnl_configuration_collection.find(filter, options).await?.try_collect::<Vec<_>>().await?;
        let cursor = pnl_configuration_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    pnl_configuration_found = Some(data); //TODO: for each configuration against a user, create a new current pnl state against it.
                    
                    // println!("Successfully fetched PnL configuration {:?}", pnl_configuration_found);
                }
                Err(e) => {
                    println!("Error while fetching PnL configuration: {}", e);
                }
            },
            Err(e) => {
                println!("Error while fetching PnL configuration: {}", e);
            }
        }
        match pnl_configuration_found {
            Some(pnl_configurations) => {
                for pnl_configuration in pnl_configurations {
                    let start_trade_date = trading_date.clone(); //TODO: update the trading date with the current date SOD
                    let end_trade_date = trading_date.clone(); //TODO: update the trading date with the current date EOD
                    let current_pnl = 0.0;
                    let current_pnl_percentage = 0.0;
                    let targeted_pnl = pnl_configuration.targeted_pnl;
                    let targeted_pnl_percentage = pnl_configuration.targeted_pnl_percentage;
                    let current_sl_hit_count = 0;
                    let max_sl_hit_count = pnl_configuration.max_sl_hit_count;
                    let current_trade_count = 0;
                    let max_trade_count = pnl_configuration.max_trade_count;
                    let current_target_hit_count = 0;
                    let max_target_hit_count = pnl_configuration.max_target_hit_count;
                    let target_hit_percentage = 0.0;
                    let max_sl_capacity = pnl_configuration.max_sl_capacity;
                    let max_trade_capital = pnl_configuration.max_trade_capital;
                    let current_used_trade_capital = 0.0;
                    let is_eligible_for_trading = true;
                    let pnl_configuration_id = pnl_configuration.id.clone();
                    let user_id = pnl_configuration.user_id.clone();
                    let (current_pnl_cache_key,current_pnl_cache_algo_types_key ) =
                        Self::get_current_pnl_cache_key(start_trade_date.as_str(), user_id.to_string().as_str());
                    let created_at = new_current_date_time_in_desired_stock_datetime_format();
                    let updated_at = new_current_date_time_in_desired_stock_datetime_format();
                    let trading_algo_types = pnl_configuration.trading_algo_types.clone();
                    let new_current_pnl_state = CurrentPnLState::new(
                        start_trade_date,
                        end_trade_date,
                        current_pnl,
                        current_pnl_percentage,
                        targeted_pnl,
                        targeted_pnl_percentage,
                        current_sl_hit_count,
                        max_sl_hit_count,
                        current_trade_count,
                        max_trade_count,
                        current_target_hit_count,
                        max_target_hit_count,
                        target_hit_percentage,
                        max_sl_capacity,
                        max_trade_capital,
                        is_eligible_for_trading,
                        "".to_string(),
                        current_used_trade_capital,
                        current_pnl_cache_key.clone(),
                        pnl_configuration_id,
                        trading_algo_types,
                        ObjectId::new(),
                        created_at,
                        updated_at,
                        user_id,
                    );

                    println!("new_current_pnl_state => {:?}", new_current_pnl_state.clone());
                    new_current_pnl_state
                        .push_current_pnl_state_to_redis_mongo(
                            current_pnl_cache_key.as_str(),
                            current_pnl_cache_algo_types_key.as_str(),
                            &current_pnl_state_collection,
                        )
                        .await;
                }
            }
            None => {
                println!("PnL configuration not found");
            }
        }
    }

    }

    pub fn fetch_current_pnl_state(
        current_pnl_cache_key: &str,
        only_via_redis: bool,
    ) -> Option<CurrentPnLState> {
        let redis_client = RedisClient::get_instance();
        let current_pnl = match redis_client.lock().unwrap().get_data(current_pnl_cache_key) {
            Ok(data) => {
                let formatted_current_pnl =
                    serde_json::from_str::<CurrentPnLState>(data.as_str()).unwrap();
                // println!("Current PnL updated => {:?}", formatted_current_pnl.clone());
                Some(formatted_current_pnl)
            }
            Err(e) => {
                println!("No cache PnL found with Error {:?}", e);

                None
            }
        };

        if only_via_redis {
            current_pnl
        } else {
            //TODO: fetch from mongodb
            None
        }
    }

    pub async fn update_current_pnl_state_via_order(
        close_order: &Order,
        current_pnl_state_collection: &Collection<CurrentPnLState>,
    ) -> () {
        let (current_pnl_cache_key, current_pnl_cache_algo_types_key ) =
            Self::get_current_pnl_cache_key(close_order.raw_stock.date.as_str(), close_order.user_id.to_string().as_str());

        let current_pnl_state_option =
            Self::fetch_current_pnl_state(current_pnl_cache_key.as_str(), true);

        if current_pnl_state_option.is_none() {
            println!("update_current_pnl_state_via_order => No current PnL state found {}", current_pnl_cache_key.as_str());
            ()
        } else {
            let mut current_pnl_state = current_pnl_state_option.unwrap();
            
            // println!("update_current_pnl_state_via_order => {:?}", current_pnl_state.clone());

            current_pnl_state.current_sl_hit_count = if close_order.is_profitable_trade {
                current_pnl_state.current_sl_hit_count
            } else {
                current_pnl_state.current_sl_hit_count + 1
            };
            current_pnl_state.current_target_hit_count = if close_order.is_profitable_trade {
                current_pnl_state.current_target_hit_count + 1
            } else {
                current_pnl_state.current_target_hit_count
            };
            current_pnl_state.current_pnl =
                current_pnl_state.current_pnl + close_order.closing_profit;

            if current_pnl_state.current_sl_hit_count >= current_pnl_state.max_sl_hit_count {
                current_pnl_state.is_eligible_for_trading = false;
                current_pnl_state.not_eligible_trading_reason =
                    "Max SL hit count reached".to_string();
            } else if current_pnl_state.current_target_hit_count
                >= current_pnl_state.max_target_hit_count
            {
                current_pnl_state.is_eligible_for_trading = false;
                current_pnl_state.not_eligible_trading_reason =
                    "Max target hit count reached".to_string();
            } else if current_pnl_state.current_pnl <= current_pnl_state.max_sl_capacity {
                current_pnl_state.is_eligible_for_trading = false;
                current_pnl_state.not_eligible_trading_reason =
                    "Max SL capacity reached".to_string();
            }
            current_pnl_state.updated_at = new_current_date_time_in_desired_stock_datetime_format();

            // println!("update_current_pnl_state_via_order => {:?}", current_pnl_state.clone());

            current_pnl_state
                .push_current_pnl_state_to_redis_mongo(
                    current_pnl_cache_key.as_str(),
                    current_pnl_cache_algo_types_key.as_str(),
                    &current_pnl_state_collection,
                )
                .await;
        }
    }

    // pub async fn update_current_pnl_state_not_eligible_trade_reason(

    //     current_pnl_state_collection: &Collection<CurrentPnLState>,
    // ) -> (){

    // }

    pub async fn push_current_pnl_state_to_redis_mongo(
        self: &Self,
        current_pnl_cache_key: &str,
        current_pnl_cache_algo_types_key: &str,
        current_pnl_state_collection: &Collection<CurrentPnLState>,
    ) {
        let current_pnl_state = self;
        println!("push_current_pnl_state_to_redis_mongo => {:?}", current_pnl_state.clone());
        let redis_client = RedisClient::get_instance();
        match redis_client.lock().unwrap().set_data(
            current_pnl_cache_key,
            serde_json::to_string(&current_pnl_state).unwrap().as_str(),
        ) {
            Ok(_) => {
                // println!(
                //     "Current PnL updated => {:?}",
                //     serde_json::to_string(&current_pnl_state).unwrap().as_str()
                // );
            }
            Err(e) => {
                println!("Not able to insert current PnL with Error {:?}", e);
            }
        }

        let filter = doc! {"current_pnl_state_cache_key": current_pnl_cache_key.clone() };
        let options: UpdateOptions = UpdateOptions::builder().upsert(true).build();
        let current_pnl_state_doc = doc! {"$set":current_pnl_state.to_document()};
        match current_pnl_state_collection
            .update_one(filter, current_pnl_state_doc, options)
            .await
        {
            Ok(_) => {
                println!("Successfully inserted/updated new current PnL state for the user with cache key: {}", current_pnl_cache_key.clone());
            }
            Err(e) => {
                println!("Error while inserting new current PnL state: {}", e);
            }
        }

        match redis_client.lock().unwrap().set_data(
            current_pnl_cache_algo_types_key,
            serde_json::to_string(&current_pnl_state.trading_algo_types).unwrap().as_str(),
        ) {
            Ok(_) => {
                println!(
                    "Current PnL algo types updated => {:?}",
                    serde_json::to_string(&current_pnl_state.trading_algo_types).unwrap().as_str()
                );
            }
            Err(e) => {
                println!("Not able to insert/update current PnL algo types with Error {:?}", e);
            }
        }


    }

    pub fn get_current_pnl_cache_key(stock_date: &str, logged_in_user: &str) -> (String, String) {
        let static_stock_date = DateTime::parse_from_str(stock_date, "%Y-%m-%d %H:%M:%S%z")
            .unwrap()
            .to_string(); //TODO: remove static date once live market data is available
        let trade_date_only =
            date_parser::return_only_date_from_datetime(static_stock_date.as_str());
        let cache_key = utils::current_pnl_state_cache_key_formatted(trade_date_only.as_str(), logged_in_user);
        let cache_key_algo_types = utils::current_pnl_state_cache_key_algotypes_formatted(trade_date_only.as_str(), logged_in_user);
        (cache_key, cache_key_algo_types)
    }

    pub fn fetch_current_pnl_algo_types_of_user(cache_key_algo_types: &str) -> Option<Vec<AlgoTypes>> {
        let redis_client = RedisClient::get_instance();
        match redis_client.lock().unwrap().get_data(cache_key_algo_types) {
            Ok(algo_types) => {
                let algo_types: Vec<AlgoTypes> = serde_json::from_str(algo_types.as_str()).unwrap();
                Some(algo_types)
            }
            Err(e) => {
                println!("Error while fetching current PnL algo types: {}", e);
                None
            }
        }
    }


    fn to_document(&self) -> Document {
        let trading_algo_types_doc = self.trading_algo_types.iter().map(|algo_type| algo_type.to_string()).collect::<Vec<String>>();
        doc! {
            "start_trade_date": self.start_trade_date.to_string(),
            "end_trade_date": self.end_trade_date.to_string(),
            "current_pnl": self.current_pnl,
            "current_pnl_percentage": self.current_pnl_percentage,
            "targeted_pnl": self.targeted_pnl,
            "targeted_pnl_percentage": self.targeted_pnl_percentage,
            "current_sl_hit_count": self.current_sl_hit_count,
            "max_sl_hit_count": self.max_sl_hit_count,
            "current_trade_count": self.current_trade_count,
            "max_trade_count": self.max_trade_count,
            "current_target_hit_count": self.current_target_hit_count,
            "max_target_hit_count": self.max_target_hit_count,
            "target_hit_percentage": self.target_hit_percentage,
            "max_sl_capacity": self.max_sl_capacity,
            "max_trade_capital": self.max_trade_capital,
            "current_used_trade_capital": self.current_used_trade_capital,
            "is_eligible_for_trading": self.is_eligible_for_trading,
            "not_eligible_trading_reason": self.not_eligible_trading_reason.to_string(),
            "current_pnl_state_cache_key": self.current_pnl_state_cache_key.to_string(),
            "pnl_configuration_id": self.pnl_configuration_id.clone(),
            "trading_algo_types": trading_algo_types_doc,
            "created_at": self.created_at.to_string(),
            "updated_at": self.updated_at.to_string(),
            "user_id": self.user_id.clone(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnLConfiguration {
    pub created_at: String,
    pub updated_at: String,
    pub start_trade_date: String,
    pub end_trade_date: String,
    pub max_trade_count: i32,

    pub symbols: Vec<String>,
    pub trading_algo_types: Vec<AlgoTypes>,

    pub max_sl_hit_count: i32,

    pub targeted_pnl: f32,
    pub targeted_pnl_percentage: f32,
    pub max_target_hit_count: i32,

    pub max_sl_capacity: f32,
    pub max_trade_capital: f32,
    pub is_active: bool,
    pub user_id: ObjectId,

    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl PnLConfiguration {
    pub fn new(
        created_at: String,
        updated_at: String,
        start_trade_date: String,
        end_trade_date: String,
        max_trade_count: i32,
        symbols: Vec<String>,
        trading_algo_types: Vec<AlgoTypes>,
        max_sl_hit_count: i32,
        targeted_pnl: f32,
        targeted_pnl_percentage: f32,
        max_target_hit_count: i32,
        max_sl_capacity: f32,
        max_trade_capital: f32,
        is_active: bool,
        user_id: ObjectId,
        id: ObjectId,
    ) -> PnLConfiguration {
        PnLConfiguration {
            created_at,
            updated_at,
            start_trade_date,
            end_trade_date,
            max_trade_count,
            symbols,
            trading_algo_types,
            max_sl_hit_count,
            targeted_pnl,
            targeted_pnl_percentage,
            max_target_hit_count,
            max_sl_capacity,
            max_trade_capital,
            is_active,
            user_id,
            id,
        }
    }

    //TODO: set the config from UI and save it in DB for the logged in user
    pub async fn new_static_config(pnl_configuration_collection: Collection<PnLConfiguration>) {
        let created_at = date_parser::new_current_date_time_in_desired_stock_datetime_format();
        let updated_at = date_parser::new_current_date_time_in_desired_stock_datetime_format();
        //TODO: remove static date once live market data is available
        let start_trade_date = DateTime::parse_from_str(CURRENT_STOCK_DATE, "%Y-%m-%d %H:%M:%S%z")
            .unwrap()
            .to_string();

        //TODO: remove static date once live market data is available
        let end_trade_date = DateTime::parse_from_str(CURRENT_STOCK_DATE, "%Y-%m-%d %H:%M:%S%z")
            .unwrap()
            .to_string();
        let max_trade_count = 5;
        let symbols = vec!["ADANIGREEN".to_string()];
        let trading_algo_types = vec![AlgoTypes::HammerPatternAlgo];
        let max_sl_hit_count = 2;
        let targeted_pnl = 1000.0;
        let targeted_pnl_percentage = 10.0;
        let max_target_hit_count = 4;
        let max_sl_capacity = -500.0;
        let max_trade_capital = 4000000.0;
        let user_id = ObjectId::from_str("64d8febebe3ea57f392c36df").unwrap();
        let id = ObjectId::new();
        let new_pnl_configuration = PnLConfiguration::new(
            created_at,
            updated_at,
            start_trade_date,
            end_trade_date,
            max_trade_count,
            symbols,
            trading_algo_types,
            max_sl_hit_count,
            targeted_pnl,
            targeted_pnl_percentage,
            max_target_hit_count,
            max_sl_capacity,
            max_trade_capital,
            true,
            user_id,
            id,
        );

        match pnl_configuration_collection
            .insert_one(new_pnl_configuration, None)
            .await
        {
            Ok(_) => {
                println!("Successfully inserted new PnL configuration");
            }
            Err(e) => {
                println!("Error while inserting new PnL configuration: {}", e);
            }
        }
    }

    //this function will run via the cron against all the user and corresponding dates
    // pub fn fetch_pnl_configuration(
    //     pnl_configuration_collection: &Collection<PnLConfiguration>,
    //     pnl_cache_key: &str,
    // ) -> Option<PnLConfiguration> {
    //     let filter = doc! {"pnl_cache_key":pnl_cache_key.to_string() };
    //     let options = FindOptions::builder().build();
    //     let cursor = pnl_configuration_collection.find(filter, options);
    //     match cursor {
    //         Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>() {
    //             Ok(data) => {
    //                 if data.len() > 0 {
    //                     Some(data[0].clone())
    //                 } else {
    //                     None
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Error while fetching PnL configuration: {}", e);
    //                 None
    //             }
    //         },
    //         Err(e) => {
    //             println!("Error while fetching PnL configuration: {}", e);
    //             None
    //         }
    //     }
    // }
}
