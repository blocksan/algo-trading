use mongodb::Collection;

use crate::common::{raw_stock::RawStock, enums::{AlgoTypes, TradeType}};
use serde::{Deserialize, Serialize};
#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeSignal{
    pub raw_stock: RawStock,
    pub trade_position_type: TradeType,
    pub trade_algo_type: AlgoTypes,
    pub trade_signal_requested_at: String,
    pub entry_price: f32,
    pub trade_sl: f32, 
    pub trade_target: f32,
    pub qty: i32,
    pub total_price: f32,
}

impl TradeSignal{
    pub fn new(raw_stock: RawStock,trade_position_type: TradeType, trade_algo_type: AlgoTypes, trade_signal_requested_at: String,  entry_price: f32, trade_sl: f32, trade_target: f32, qty: i32, total_price: f32 ) -> TradeSignal {
        TradeSignal {
            raw_stock,
            trade_position_type,
            trade_algo_type,
            trade_signal_requested_at,
            entry_price,
            trade_sl,
            trade_target,
            qty,
            total_price,
        }
    }

    pub fn get_raw_stock(&self) -> &RawStock {
        &self.raw_stock
    }

    pub fn get_trade_algo_type(&self) -> &AlgoTypes {
        &self.trade_algo_type
    }

    pub fn get_trade_signal_requested_at(&self) -> &String {
        &self.trade_signal_requested_at
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
        //TODO: add to the database as well
        
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