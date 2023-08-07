use mongodb::{Database, Collection};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::{Arc, Mutex}};

use crate::{algo_hub::hammer_pattern::{HammerCandle, HammerPatternUtil}, data_consumer::current_market_state::CurrentMarketState, order_manager::{order_dispatcher::Order, trade_signal_keeper::{TradeSignal, TradeSignalsKeeper}, self}};


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlgoTypes{
    HammerPatternAlgo,
    ShootingStarPatternAlgo,
}

impl fmt::Display for AlgoTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeType {
    Long,
    Short,
}

impl fmt::Display for TradeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarketTrend{
    Bullish,
    Bearish,
    Sideways,
    Flat,
}

impl fmt::Display for MarketTrend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeFrame{
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    OneDay,
    OneWeek,
    OneMonth,
    OneYear,
    Infinity,
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThreadJobType{
    DataConsumerViaSocket,
    TradeWatcherCron
}

#[derive(Debug, Clone)]
pub struct ThreadWorkerConfig {
    pub thread_job_type: ThreadJobType,
    pub root_system_config: RootSystemConfig,
    pub time_frame: TimeFrame,
}

#[derive(Debug, Clone)]
pub struct TradeableAlgoConfig<GenericCollectionStruct, GenericCollectionLedger> {
    pub algo_type: AlgoTypes,
    pub algo_db_collection: Collection<GenericCollectionStruct>,
    pub algo_inmemory_ledger: GenericCollectionLedger,
}

#[derive(Debug, Clone)]
pub struct RootSystemConfig {
    pub database_instance : Database,
    pub hammer_candle_collection: Collection<HammerCandle>,
    pub hammer_ledger: HammerPatternUtil,
    pub current_market_state_collection: Collection<CurrentMarketState>,
    pub orders_collection: Collection<Order>,
    pub trade_signal_collection: Collection<TradeSignal>,
    pub server_url: String,
    pub tradeable_algo_types: Vec<AlgoTypes>,
    pub trade_keeper: TradeSignalsKeeper, 
    pub order_manager: order_manager::order_dispatcher::OrderManager,
    pub shared_order_ledger: Arc<Mutex<Vec<Order>>>
}
