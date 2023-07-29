use serde::{Deserialize, Serialize};
use std::fmt;


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
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
pub struct ServerUrlTimeFrame {
    pub server_url: String,
    pub time_frame: TimeFrame,
}
