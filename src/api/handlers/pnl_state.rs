use actix_web::{post, HttpResponse, Responder};
use crate::order_manager::pnl_state::{CurrentPnLState, PnLConfiguration};

#[post("/fetch_current_pnl")]
async fn fetch_current_pnl() -> impl Responder {
    println!("fetch_current_pnl");
    let current_cache_key = "CPnL_2022_10_18_64d8febebe3ea57f392c36df";
    let only_redis = true;
   let current_pnl_state_option = CurrentPnLState::fetch_current_pnl_state(current_cache_key, only_redis);
   
   if current_pnl_state_option.is_some(){
        let current_pnl_state = current_pnl_state_option.unwrap();
       HttpResponse::Ok().json(current_pnl_state)
   }else{
       HttpResponse::Ok().body("No Pnl")
   }
}

#[post("/add_new_pnl_configuration")]
async fn add_new_pnl_configuration() -> impl Responder {
    PnLConfiguration::new_pnl_static_config_via_db().await;
    println!("add_new_pnl_configuration");
    HttpResponse::Ok().body("add_new_pnl_configuration")
}