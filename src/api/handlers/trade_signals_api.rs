use actix_web::{post, HttpResponse, Responder, web};
use crate::{ api::utils::app_state::AppState, order_manager:: trade_signal_keeper::{TradeSignal, TradeSignalBodyParams}};

#[post("/fetch_trade_signals")]
async fn fetch_trade_signals(_app_state: web::Data<AppState>, trade_signals_body_params: web::Json<TradeSignalBodyParams>) -> impl Responder {
    println!("fetch_trade_signals body: {:?}", trade_signals_body_params);
    let order_options = TradeSignal::fetch_trade_signals(trade_signals_body_params.into_inner()).await;
 
    if order_options.is_some(){
         let orders = order_options.unwrap();
        HttpResponse::Ok().json(orders)
    }else{
        HttpResponse::Ok().body("No orders found")
    }
}
