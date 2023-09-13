use serde::{Serialize, Deserialize};

use crate::common::enums::AlgoTypes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HammerPatternAlgoConfiguration{

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShootingStarPatternAlgoConfiguration{

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlgoConfigurationMetadata{
    HammerPatternMetadata(HammerPatternAlgoConfiguration),
    ShootingStarPatternMetadata(ShootingStarPatternAlgoConfiguration)
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgoConfigurator{
    pub algo_type: AlgoTypes,
    pub algo_metadata: AlgoConfigurationMetadata
}

impl AlgoConfigurator{
    // pub fn new(algo_type: AlgoTypes) -> AlgoConfigurator{
    //     AlgoConfigurator{
    //         algo_type,
    //         algo_metadata: Self.configure()
    //     }
    // }

    pub fn initiate_the_backtest(&self){
        match &self.algo_metadata{
            AlgoConfigurationMetadata::HammerPatternMetadata(configuration) => {
                println!("Hammer Pattern Algo Configuration received {:?} =>",configuration);
            },
            AlgoConfigurationMetadata::ShootingStarPatternMetadata(configuration) => {
                println!("Shooting Star Pattern Algo Configuration received {:?} =>",configuration);
            }
        }
    }
}