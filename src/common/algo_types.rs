use std::fmt;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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