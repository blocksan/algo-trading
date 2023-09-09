use actix_web::{post, HttpResponse, Responder, web};
use crate::algo_hub::hammer_pattern::{HammerCandle, HammerCandleBodyParams};

#[post("/algo/run_hammer_pattern")]
async fn run_hammer_pattern(hammer_candles_body_params: web::Json<HammerCandleBodyParams>) -> impl Responder {
    println!("fetch_hammer_candles body: {:?}", hammer_candles_body_params);
    let hammer_candles_option = HammerCandle::fetch_hammer_candles(hammer_candles_body_params.into_inner()).await;
 
    if hammer_candles_option.is_some(){
         let hammer_candles = hammer_candles_option.unwrap();
        HttpResponse::Ok().json(hammer_candles)
    }else{
        HttpResponse::Ok().body("No hammer candles found")
    }
}