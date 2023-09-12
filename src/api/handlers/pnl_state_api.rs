use actix_web::{post, HttpResponse, Responder, web, get};
use crate::order_manager::pnl_state::{CurrentPnLState, PnLConfiguration, CurrentPnLStateBodyParams};


#[post("/fetch_current_pnl_state")]
async fn fetch_current_pnl_state(current_pnl_state_patams: web::Json<CurrentPnLStateBodyParams>) -> impl Responder {
    println!("fetch_current_pnl_state");
    let only_redis = false;
   let current_pnl_state_option = CurrentPnLState::fetch_current_pnl_state(current_pnl_state_patams.into_inner(), only_redis).await;
   
   if current_pnl_state_option.is_some(){
        let current_pnl_state = current_pnl_state_option.unwrap();
       HttpResponse::Ok().json(current_pnl_state)
   }else{
       HttpResponse::Ok().body("No current Pnl state found")
   }
}






#[post("/add_new_pnl_configuration")]
async fn add_new_pnl_configuration() -> impl Responder {
    PnLConfiguration::new_static_backtest_config().await;
    println!("add_new_pnl_configuration");
    HttpResponse::Ok().body("add_new_pnl_configuration")
}

#[get("/fetch_current_pnl_configuration/{user_id}")]
async fn fetch_current_pnl_configuration(path: web::Path<String>) -> impl Responder {
    println!("fetch_current_pnl_configuration");
    let user_id = path.into_inner();
    let current_pnl_configuration_option = PnLConfiguration::fetch_current_pnl_configuration(None, Some(user_id), None).await;
    if current_pnl_configuration_option.is_some(){
        let current_pnl_configuration = current_pnl_configuration_option.unwrap();
        HttpResponse::Ok().json(current_pnl_configuration)
    }else{
        HttpResponse::Ok().body("No PnL configuration found")
    }
}