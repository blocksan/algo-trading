use actix_web::{post, HttpResponse, Responder, web};
use crate::{ api::utils::app_state::AppState, algo_hub::hammer_pattern::{HammerCandle, HammerCandleBodyParams}};

#[post("/candles/fetch_hammer_candles")]
async fn fetch_hammer_candles(_app_state: web::Data<AppState>, hammer_candles_body_params: web::Json<HammerCandleBodyParams>) -> impl Responder {
    println!("fetch_hammer_candles body: {:?}", hammer_candles_body_params);
    let hammer_candles_option = HammerCandle::fetch_hammer_candles(hammer_candles_body_params.into_inner()).await;
 
    if hammer_candles_option.is_some(){
         let hammer_candles = hammer_candles_option.unwrap();
        HttpResponse::Ok().json(hammer_candles)
    }else{
        HttpResponse::Ok().body("No hammer candles found")
    }
}
