use mongodb::{Collection, bson::{oid::ObjectId, doc}, options::FindOptions};

use crate::{common::{raw_stock::RawStock, enums::{AlgoTypes, TradeType, TimeFrame}, date_parser}, config::mongodb_connection};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;

const QTY:i32 = 10;

#[derive(Deserialize, Debug)]
pub struct TradeSignalBodyParams{
    pub start_trade_date: String,
    pub end_trade_date: String,
    pub trade_position_type: Option<TradeType>,
    pub trade_algo_type: Option<AlgoTypes>,
    pub symbol: Option<String>,
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeSignal{
    pub raw_stock: RawStock,
    pub trade_position_type: TradeType,
    pub trade_algo_type: AlgoTypes,
    pub created_at: String,
    pub entry_price: f32,
    pub trade_sl: f32, 
    pub trade_target: f32,
    pub qty: i32,
    pub total_price: f32,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub algo_id: ObjectId,
}

impl TradeSignal{
    pub fn new(raw_stock: RawStock,trade_position_type: TradeType, trade_algo_type: AlgoTypes, created_at: String,  entry_price: f32, trade_sl: f32, trade_target: f32, qty: i32, total_price: f32, id: ObjectId, algo_id: ObjectId ) -> TradeSignal {
        TradeSignal {
            raw_stock,
            trade_position_type,
            trade_algo_type,
            created_at,
            entry_price,
            trade_sl,
            trade_target,
            qty,
            total_price,
            id,
            algo_id
        }
    }

    pub fn get_raw_stock(&self) -> &RawStock {
        &self.raw_stock
    }

    pub fn get_trade_algo_type(&self) -> &AlgoTypes {
        &self.trade_algo_type
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn create_trade_signal(symbol: String, date: String, close: f32, high:f32, low:f32, open:f32, volume:i32, market_time_frame: TimeFrame, trade_position_type: TradeType, algo_type: AlgoTypes, entry_price: f32, trade_sl: f32, trade_target: f32, algo_id: ObjectId ) -> Option<TradeSignal> {
        let trade_signal = TradeSignal::new(
            RawStock::new(
                symbol,
                date,
                close,
                high,
                low,
                open,
                volume,
                market_time_frame,
            ),
            trade_position_type,
            algo_type,
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            entry_price,
            trade_sl,
            trade_target,
            QTY,
            entry_price*QTY as f32,
            ObjectId::new(),
            algo_id

        );
        Some(trade_signal)
    }

    pub async fn fetch_trade_signals(fetch_trade_signals_params: TradeSignalBodyParams) -> Option<Vec<TradeSignal>> {
        let TradeSignalBodyParams {
            start_trade_date,
            end_trade_date,
            trade_position_type,
            trade_algo_type,
            symbol,
        } = fetch_trade_signals_params;
        let mut filter = doc! {
            "created_at": {
                "$gte": start_trade_date,
                "$lte": end_trade_date
            },
        }; 

        if trade_position_type.is_some() {
            filter.insert("trade_position_type", trade_position_type.unwrap().to_string());
        }

        if trade_algo_type.is_some() {
            filter.insert("trade_algo_type", trade_algo_type.unwrap().to_string());
        }

        if symbol.is_some() {
            filter.insert("raw_stock.symbol", symbol.unwrap());
        }

        let options = FindOptions::builder().build();
        let trade_signal_collection = TradeSignal::get_trade_signal_collection().await;

        // let cursor = pnl_configuration_collection.find(filter, options).await?.try_collect::<Vec<_>>().await?;
        let cursor = trade_signal_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    // println!("Successfully fetched PnL configuration {:?}", pnl_configuration_found);
                    Some(data) 
                }
                Err(e) => {
                    println!("Cursor Error fetch_trade_signals:  {}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error fetch_trade_signals: {}", e);
                None
            }
        }
    }

    pub async fn get_trade_signal_collection() -> Collection<TradeSignal> {
        let db = mongodb_connection::fetch_db_connection().await;
        let trade_signal_collection_name = "trade_signals";
        let trade_signal_collection = db.collection::<TradeSignal>(trade_signal_collection_name);
        trade_signal_collection
    }

}

#[derive(Debug, Clone, Default)]
pub struct TradeSignalsKeeper{
    trade_signals: Vec<TradeSignal>,
}

impl TradeSignalsKeeper{
    pub fn new() -> TradeSignalsKeeper {
        TradeSignalsKeeper {
            trade_signals: Vec::new(),
        }
    }

    pub async fn add_trade_signal(&mut self, trade_signal: &TradeSignal) {
        let trade_signal_collection = TradeSignal::get_trade_signal_collection().await;
        match trade_signal_collection.insert_one(trade_signal.clone(), None).await{
            Ok(_) => {
                println!("Trading signal added to the database");
            },
            Err(e) => {
                println!("Error while adding trading signal to the database {}", e);
            }
        }

        self.trade_signals.push(trade_signal.clone());

    }

    pub fn get_trade_signals(&self) -> &Vec<TradeSignal> {
        &self.trade_signals
    }

    

}