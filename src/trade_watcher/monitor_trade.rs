use std::sync::{Mutex, Arc};

use mongodb::Collection;

use crate::{order_manager::order_dispatcher::{OrderManager, Order}, common::{raw_stock::RawStock, enums::TradeType, redis_client::RedisClient}};


pub async fn check_for_exit_opportunity(order_manager: &mut OrderManager, stock: RawStock, redis_client: &Mutex<RedisClient>, order_collection: Collection<Order>, shared_order_ledger: Arc<Mutex<Vec<Order>>>){
    // let orders = order_manager.get_orders().clone();
    let mut shared_orders = shared_order_ledger.lock().unwrap().clone();
    let mut exit_price = 0.0;
    // println!("orders currently open: {:?} with number {}", shared_orders, shared_orders.len());
    for (index, order) in shared_orders.iter_mut().enumerate(){
        exit_price = 0.0;
        if order.symbol == stock.symbol && order.is_trade_open{
            if order.trade_position_type == TradeType::Long{
                if stock.low <= order.trade_sl{
                    exit_price = stock.low;
                    // order.exit_trade(stock.low, new_current_date_time_in_desired_stock_datetime_format());
                }
                else if stock.high >= order.trade_target{
                    exit_price = stock.high;
                    // order.exit_trade(stock.high, new_current_date_time_in_desired_stock_datetime_format());
                }
            }
            else if order.trade_position_type == TradeType::Short{
                if stock.high >= order.trade_sl{
                    exit_price = stock.high;
                    // order.exit_trade(stock.high, new_current_date_time_in_desired_stock_datetime_format());
                }
                else if stock.low <= order.trade_target{
                    exit_price = stock.low;
                    // order.exit_trade(stock.low, new_current_date_time_in_desired_stock_datetime_format());
                }
            }
            if exit_price > 0.0{

                let updated_order = order_manager.exit_and_update_order( order, exit_price, redis_client, &order_collection).await;
                // println!("updated order: {:?} for index {}", updated_order, index);
                if updated_order.is_some(){
                    shared_orders[index] = updated_order.unwrap();
                }
                println!("shared order after update at index: {:?} values => {:?}",index, shared_orders[index]);
                break;
            }
        }
    }
}