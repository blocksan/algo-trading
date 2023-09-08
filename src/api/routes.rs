use actix_web::{web, get, Responder, HttpResponse};
use super::handlers::{
    pnl_state::{fetch_current_pnl, add_new_pnl_configuration}, 
    current_market_state::fetch_current_market_state, 
    orders::fetch_orders, user::{create_user, fetch_user}};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Happy Trading!")
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(healthcheck)
            .service(fetch_current_pnl)
            .service(add_new_pnl_configuration)
            .service(fetch_current_market_state)
            .service(fetch_orders)
            .service(create_user)
            .service(fetch_user)
    );
}