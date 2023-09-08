use actix_web::{post, HttpResponse, Responder, web};
use serde::Deserialize;
use crate::{ api::utils::app_state::AppState, data_consumer::current_market_state::CurrentMarketState};

#[derive(Deserialize, Debug)]
struct FetchOrdersBodyParams{
    user_id: String,
}

#[post("/fetch_orders")]
async fn fetch_orders(app_state: web::Data<AppState>, body: web::Json<FetchOrdersBodyParams>) -> impl Responder {
    println!("fetch_orders body: {:?}", body);
    let current_cache_key = body.user_id.clone();
    let current_market_state_collection = &app_state.current_market_state_collection;
   let current_market_state_option = CurrentMarketState::api_fetch_previous_market_state(current_cache_key.as_str(), current_market_state_collection).await;
   
   if current_market_state_option.is_some(){
        let current_market_state = current_market_state_option.unwrap();
       HttpResponse::Ok().json(current_market_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}
