use actix_web::{post, HttpResponse, Responder, web};
use crate::{ api::utils::app_state::AppState, order_manager::order_dispatcher::{OrderBodyParams, OrderManager}};

#[post("/fetch_orders")]
async fn fetch_orders(_app_state: web::Data<AppState>, order_body_params: web::Json<OrderBodyParams>) -> impl Responder {
    println!("fetch_orders body: {:?}", order_body_params);
    let order_options = OrderManager::fetch_orders(order_body_params.into_inner()).await;
 
    if order_options.is_some(){
         let orders = order_options.unwrap();
        HttpResponse::Ok().json(orders)
    }else{
        HttpResponse::Ok().body("No orders found")
    }
}
