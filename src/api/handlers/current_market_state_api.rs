use actix_web::{HttpResponse, post, web, Responder};
use crate::{api::utils::app_state::AppState, data_consumer::current_market_state::{CurrentMarketState, CurrentMarketStateBodyParams}};

#[post("/fetch_current_market_state")]
async fn fetch_current_market_state(_app_state: web::Data<AppState>, current_market_state_body_params: web::Json<CurrentMarketStateBodyParams>) -> impl Responder {
    println!("fetch_current_market_state body: {:?}", current_market_state_body_params);
   let only_via_redis = false;   
   let current_market_state_option = CurrentMarketState::fetch_current_market_states(current_market_state_body_params.into_inner(), only_via_redis).await;

   if current_market_state_option.is_some(){
        let current_market_state = current_market_state_option.unwrap();
       HttpResponse::Ok().json(current_market_state)
   }else{
       HttpResponse::Ok().body("No current_market_state found")
   }
}