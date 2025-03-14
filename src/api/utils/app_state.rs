use mongodb::Collection;

use crate::data_consumer::current_market_state::CurrentMarketState;

pub struct AppState{
    pub current_market_state_collection: Collection<CurrentMarketState>,
}