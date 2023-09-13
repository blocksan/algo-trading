use actix_web::{web, get, Responder, HttpResponse};

use super::handlers::{
    pnl_state_api::{add_new_pnl_configuration, fetch_current_pnl_state, fetch_current_pnl_configuration}, 
    current_market_state_api::fetch_current_market_state, 
    order_api::fetch_orders, user_api::{create_user, fetch_user}, trade_signals_api::fetch_trade_signals, candles_api::fetch_hammer_candles, 
    initiate_algo_api::{create_static_pnl_config, backtest_strategy}};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Happy Trading!")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(healthcheck)
        .service(create_user)
        .service(fetch_user)
        //TODO: .service(update_user_details) 
        .service(add_new_pnl_configuration)
        .service(fetch_current_pnl_configuration)
            .service(fetch_current_pnl_state)
            .service(fetch_current_market_state)
            .service(fetch_orders)
            .service(fetch_trade_signals)
            .service(fetch_hammer_candles)
            .service(create_static_pnl_config)
            .service(backtest_strategy)
    );
}