use std::fmt;
#[derive(Debug, Clone, PartialEq)]
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