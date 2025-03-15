use actix_web::{post, HttpResponse, Responder, web, get};
use crate::{algo_hub::{algo_backtester::AlgoBacktester, algo_runner::create_curren_pnl_states}, api::utils::api_response::ApiResponse, common::constant::{IS_BACKTEST_CONFIG, MAX_SL_CAPACITY, MAX_SL_HIT_COUNT, MAX_TARGET_HIT_COUNT, MAX_TRADE_CAPITAL, MAX_TRADE_COUNT, REQUESTED_END_DATE, REQUESTED_START_DATE, REQUESTED_SYMBOL, REQUESTED_USER_ID, TARGETED_PNL, TARGETED_PNL_PERCENTAGE}, order_manager::pnl_state::{CurrentPnLState, CurrentPnLStateBodyParams, PnLConfiguration}};


#[post("/fetch_current_pnl_state")]
async fn fetch_current_pnl_state(current_pnl_state_params: web::Json<CurrentPnLStateBodyParams>) -> impl Responder {
    println!("fetch_current_pnl_state");
    let only_redis = false;
   let current_pnl_state_option = CurrentPnLState::fetch_current_pnl_state(current_pnl_state_params.into_inner(), only_redis).await;
   
   if current_pnl_state_option.is_some(){
        let current_pnl_state = current_pnl_state_option.unwrap();
       HttpResponse::Ok().json(current_pnl_state)
   }else{
       HttpResponse::Ok().body("No current Pnl state found")
   }
}

#[post("/algo/create_static_pnl_config/backtest_strategy")]
async fn create_static_pnl_config() -> impl Responder {
    // let mut result: Option<String> = None;
    println!("create_static_pnl_config API");
    let result = AlgoBacktester::create_static_pnl_config().await;

    if result.is_some(){
        // let pnl_configurations = result.clone().unwrap();
        let current_pnl_state_created = create_curren_pnl_states(result.clone()).await;
        if current_pnl_state_created.is_some(){
            return HttpResponse::Ok().json(current_pnl_state_created.unwrap())
        }else{
            return HttpResponse::Ok().body("Current PnL states not created")
        }
        // HttpResponse::Ok().json(result.unwrap())
    }else{
        HttpResponse::Ok().body("No hammer candles found")
    }
}




#[post("/add_new_pnl_configuration")]
async fn add_new_pnl_configuration() -> impl Responder {
    let requested_start_date = REQUESTED_START_DATE;
    let requested_end_date = REQUESTED_END_DATE;
    let requested_symbol = REQUESTED_SYMBOL;
    let requested_user_id = REQUESTED_USER_ID;
    let max_trade_count = MAX_TRADE_COUNT;
    let max_sl_hit_count = MAX_SL_HIT_COUNT;
    let targeted_pnl = TARGETED_PNL;
    let targeted_pnl_percentage = TARGETED_PNL_PERCENTAGE;
    let max_target_hit_count = MAX_TARGET_HIT_COUNT;
    let max_sl_capacity = MAX_SL_CAPACITY;
    let max_trade_capital = MAX_TRADE_CAPITAL;
    let is_backtest_config = IS_BACKTEST_CONFIG;

    let (new_pnl_configuration, message) = PnLConfiguration::new_static_backtest_config(
        requested_start_date,
        requested_end_date,
        requested_symbol,
        requested_user_id,
        max_trade_count,
        max_sl_hit_count,
        targeted_pnl,
        targeted_pnl_percentage,
        max_target_hit_count,
        max_sl_capacity,
        max_trade_capital,
        is_backtest_config,
    ).await;
    if new_pnl_configuration.is_some(){
        let new_pnl_configuration = new_pnl_configuration.unwrap();
        ApiResponse::success("add_new_pnl_configuration", serde_json::json!({
            "pnl_configuration": new_pnl_configuration
        }))
    }else{
        ApiResponse::error(message)
    }
}

#[get("/fetch_current_pnl_configuration/{user_id}")]
async fn fetch_current_pnl_configuration(path: web::Path<String>) -> impl Responder {
    println!("fetch_current_pnl_configuration");
    let user_id = path.into_inner();
    let current_pnl_configuration_option = PnLConfiguration::fetch_current_pnl_configuration(None, Some(user_id), None, None).await;
    if current_pnl_configuration_option.is_some(){
        let current_pnl_configuration = current_pnl_configuration_option.unwrap();
        HttpResponse::Ok().json(current_pnl_configuration)
    }else{
        HttpResponse::Ok().body("No PnL configuration found")
    }
}