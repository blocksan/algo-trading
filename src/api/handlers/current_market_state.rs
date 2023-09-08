use actix_web::{HttpResponse, post, web, Responder};
use serde::Deserialize;
use crate::{api::utils::app_state::AppState, data_consumer::current_market_state::CurrentMarketState};

#[derive(Deserialize, Debug)]
struct CurrentMarketStateBodyParams{
    current_market_cache_key: String,
}

#[post("/fetch_current_market_state")]
async fn fetch_current_market_state(app_state: web::Data<AppState>, body: web::Json<CurrentMarketStateBodyParams>) -> impl Responder {
    println!("fetch_current_market_state body: {:?}", body);
    let current_cache_key = body.current_market_cache_key.clone();
    let current_market_state_collection = &app_state.current_market_state_collection;
   let current_market_state_option = CurrentMarketState::api_fetch_previous_market_state(current_cache_key.as_str(), current_market_state_collection).await;
   
   if current_market_state_option.is_some(){
        let current_market_state = current_market_state_option.unwrap();
       HttpResponse::Ok().json(current_market_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}