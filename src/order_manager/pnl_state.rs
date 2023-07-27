use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::FindOptions,
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::common::{date_parser, enums::AlgoTypes};
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
    pub max_risk_capacity: i64,
    pub trade_capital: i64,
}

impl CurrentPnLState {
    pub fn new(
        start_trade_date: String,
        end_trade_date: String,
        current_pnl: f32,
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
        max_risk_capacity: i64,
        trade_capital: i64,
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
            max_risk_capacity,
            trade_capital,
        }
    }

    pub async fn new_static_current_pnl_state(
        current_pnl_state_collection: Collection<CurrentPnLState>,
        pnl_configuration_collection: Collection<PnLConfiguration>,
    ) {
        let filter = doc! {};
        let options = FindOptions::builder().limit(1).build();

        let mut pnl_configuration_found: Option<PnLConfiguration> = None;
        // let cursor = pnl_configuration_collection.find(filter, options).await?.try_collect::<Vec<_>>().await?;
        let cursor = pnl_configuration_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    pnl_configuration_found = Some(data[0].clone().into());
                    println!("Successfully fetched PnL configuration");
                }
                Err(e) => {
                    println!("Error while fetching PnL configuration: {}", e);
                }
            },
            Err(e) => {
                println!("Error while fetching PnL configuration: {}", e);
            }
        }
        // .try_collect::<Vec<_>>().await?;

        // while let Some(result) = cursor.next().await {
        //     match result {
        //         Ok(doc) => {
        //             let document: Document = mongodb::bson::from_document(doc as PnLConfiguration)?;
        //             document.to_string();
        //             println!("{:?}", document);
        //         }
        //         Err(e) => eprintln!("Error while reading: {:?}", e),
        //     }
        // }
        match pnl_configuration_found {
            Some(pnl_configuration) => {
                let start_trade_date = pnl_configuration.start_trade_date;
                let end_trade_date = pnl_configuration.end_trade_date;
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
                let max_risk_capacity = pnl_configuration.max_risk_capacity;
                let trade_capital = pnl_configuration.trade_capital;

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
                    max_risk_capacity,
                    trade_capital,
                );

                match current_pnl_state_collection
                    .insert_one(new_current_pnl_state, None)
                    .await
                {
                    Ok(_) => {
                        println!("Successfully inserted new current PnL state");
                    }
                    Err(e) => {
                        println!("Error while inserting new current PnL state: {}", e);
                    }
                }
            }
            None => {
                println!("PnL configuration not found");
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnLConfiguration {
    pub created_at: String,
    pub start_trade_date: String,
    pub end_trade_date: String,
    pub max_trade_count: i32,

    pub symbols: Vec<String>,
    pub trading_algo_types: Vec<AlgoTypes>,

    pub max_sl_hit_count: i32,

    pub targeted_pnl: f32,
    pub targeted_pnl_percentage: f32,
    pub max_target_hit_count: i32,

    pub max_risk_capacity: i64,
    pub trade_capital: i64,
}

impl PnLConfiguration {
    pub fn new(
        created_at: String,
        start_trade_date: String,
        end_trade_date: String,
        max_trade_count: i32,
        symbols: Vec<String>,
        trading_algo_types: Vec<AlgoTypes>,
        max_sl_hit_count: i32,
        targeted_pnl: f32,
        targeted_pnl_percentage: f32,
        max_target_hit_count: i32,
        max_risk_capacity: i64,
        trade_capital: i64,
    ) -> PnLConfiguration {
        PnLConfiguration {
            created_at,
            start_trade_date,
            end_trade_date,
            max_trade_count,
            symbols,
            trading_algo_types,
            max_sl_hit_count,
            targeted_pnl,
            targeted_pnl_percentage,
            max_target_hit_count,
            max_risk_capacity,
            trade_capital,
        }
    }

    pub async fn new_static_config(pnl_configuration_collection: Collection<PnLConfiguration>) {
        let created_at = date_parser::new_current_date_time_in_desired_stock_datetime_format();
        let start_trade_date =
            date_parser::new_current_date_time_in_desired_stock_datetime_format();
        let end_trade_date = date_parser::new_current_date_time_in_desired_stock_datetime_format();
        let max_trade_count = 5;
        let symbols = vec!["ADANIGREEN".to_string()];
        let trading_algo_types = vec![AlgoTypes::HammerPatternAlgo];
        let max_sl_hit_count = 2;
        let targeted_pnl = 1000.0;
        let targeted_pnl_percentage = 10.0;
        let max_target_hit_count = 4;
        let max_risk_capacity = 500;
        let trade_capital = 10000;
        let new_pnl_configuration = PnLConfiguration::new(
            created_at,
            start_trade_date,
            end_trade_date,
            max_trade_count,
            symbols,
            trading_algo_types,
            max_sl_hit_count,
            targeted_pnl,
            targeted_pnl_percentage,
            max_target_hit_count,
            max_risk_capacity,
            trade_capital,
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
}
