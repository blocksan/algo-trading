use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
pub mod order_manager;
use order_manager::{
    order_dispatcher,
    pnl_state::{self, CurrentPnLState, PnLConfiguration},
    trade_signal_keeper::{self, TradeSignal},
};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/fetch_current_pnl")]
async fn fetch_current_pnl() -> impl Responder {
    CurrentPnLState::fetch_current_pnl();
    HttpResponse::Ok().body("Current Pnl")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(fetch_current_pnl)
    
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}