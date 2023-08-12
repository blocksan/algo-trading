use mongodb::{Collection, bson::oid::ObjectId};

use crate::common::{raw_stock::RawStock, enums::{AlgoTypes, TradeType, TimeFrame}, date_parser};
use serde::{Deserialize, Serialize};

const QTY:i32 = 10;

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

    pub async fn add_trade_signal(&mut self, trade_signal: &TradeSignal, trade_signal_collection: Collection<TradeSignal>) {
        
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