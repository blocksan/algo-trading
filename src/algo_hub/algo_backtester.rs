use serde::{Serialize, Deserialize};

use crate::{algo_hub::algo_runner::backtest_strategy, order_manager::pnl_state::PnLConfiguration};

use super::algo_runner::{create_static_pnl_config, create_curren_pnl_states};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct AlgoBacktester{
    pub pnl_configuration_id: String
}

impl AlgoBacktester {

    pub fn new(pnl_configuration_id: String) -> AlgoBacktester{
        AlgoBacktester{
            pnl_configuration_id
        }
    }

    pub async fn initiate_the_backtest(&self) -> Option<String>{
        println!("AlgoBacktester initiated with pnl_configuration_id: {}", self.pnl_configuration_id);
        let result = backtest_strategy(self.pnl_configuration_id.clone()).await;
        println!("AlgoBacktester result: {:?}", result);
        None
    }

    pub async fn create_static_pnl_config() -> Option<Vec<PnLConfiguration>>{
        let result = create_static_pnl_config().await;
        result
    }

    pub async fn create_curren_pnl_states(pnl_configurations: Option<Vec<PnLConfiguration>>) -> Option<String> {
        let result = create_curren_pnl_states(pnl_configurations).await;
        result
    }


}

