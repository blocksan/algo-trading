use std::sync::Mutex;

use chrono::NaiveTime;
use mongodb::Collection;

use crate::{order_manager::{order_dispatcher::{OrderManager, Order}, pnl_state::CurrentPnLState}, common::{raw_stock::RawStock, enums::TradeType, redis_client::RedisClient, date_parser::{if_first_time_greater_than_second_time, return_only_time_from_datetime}}};

const THRESHOLD_TRADE_END_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(15, 15, 0);

pub async fn check_for_exit_opportunity(order_manager: &mut OrderManager, stock: RawStock, redis_client: &Mutex<RedisClient>, order_collection: Collection<Order>, current_pnl_collection: Collection<CurrentPnLState>, shared_order_ledger: &mut Vec<Order>){
    // let orders = order_manager.get_orders().clone();
    // let mut shared_orders = shared_order_ledger.lock().unwrap().clone();
    println!("");
    println!("");
    println!("orders currently open: {:?} with number {}", shared_order_ledger, shared_order_ledger.len());
    println!("");
    println!("");
    let mut closed_order_indexes: Vec<usize> = Vec::new();
    for (index, order) in shared_order_ledger.iter_mut().enumerate(){
        let mut exit_price = 0.0;
        if order.symbol == stock.symbol && order.is_trade_open && order.is_trade_executed{
            if order.trade_position_type == TradeType::Long{
                if stock.close <= order.trade_sl{
                    exit_price = stock.close;
                    // order.exit_trade(stock.low, new_current_date_time_in_desired_stock_datetime_format());
                }
                else if stock.close >= order.trade_target{
                    exit_price = stock.close;
                    // order.exit_trade(stock.high, new_current_date_time_in_desired_stock_datetime_format());
                }
            }
            else if order.trade_position_type == TradeType::Short{
                if stock.close >= order.trade_sl{
                    exit_price = stock.close;
                    // order.exit_trade(stock.high, new_current_date_time_in_desired_stock_datetime_format());
                }
                else if stock.close <= order.trade_target{
                    exit_price = stock.close;
                    // order.exit_trade(stock.low, new_current_date_time_in_desired_stock_datetime_format());
                }
            }
            let closed_order = if exit_price > 0.0{

                order_manager.close_executed_order( order, exit_price, redis_client, &order_collection).await
                // println!("updated order: {:?} for index {}", closed_order, index);
                
                
                // break;
            }else if if_first_time_greater_than_second_time( Some(return_only_time_from_datetime(Some(stock.date.clone()))), THRESHOLD_TRADE_END_TIME){
               order_manager.close_executed_order( order, stock.close, redis_client, &order_collection).await
                // println!("updated order: {:?} for index {}", closed_order, index);
                
                // println!("shared order after update at index: {:?} values => {:?}",index, shared_orders[index]);
                // break;
            }else{
                None
            };
            if closed_order.is_some(){
                let temp_order = closed_order.unwrap();
                order.is_profitable_trade = temp_order.is_profitable_trade;
                order.is_trade_open = temp_order.is_trade_open;
                order.closing_profit = temp_order.closing_profit;
                order.order_closed_at = temp_order.order_closed_at;
                order.is_profitable_trade = temp_order.is_profitable_trade;
                println!();
                println!("#################");
                println!("order {:?} closed for stock", order.order_id);
                println!("#################");
                println!();

                CurrentPnLState::update_current_pnl_state(order, &current_pnl_collection).await;

                closed_order_indexes.push(index);
                //TODO: phase 2-3 delete the order from the shared order ledger once the order is closed
                // challenge is that we are iterating over the shared order ledger and we cannot delete the order from the ledger while iterating over it.
                // we can use a for loop and iterate over the shared order ledger and delete the order from the ledger once the order is closed but then it will add another O(n) complexity to the code.
            }
        } else if !order.is_trade_executed{ //this else block will not be present when Zerodha API is integrated for order execution
            if stock.open >= order.entry_price && order.trade_position_type == TradeType::Long{
                let updated_order = order_manager.mark_order_executed( order, stock.open, redis_client, &order_collection).await;
                // println!("updated order: {:?} for index {}", updated_order, index);
                if updated_order.is_some(){
                    let temp_order = updated_order.unwrap();
                    order.is_trade_executed = true;
                    order.entry_price = temp_order.entry_price;
                    order.order_executed_at = temp_order.order_executed_at;
                    println!();
                    println!("#################");
                    println!("order {:?} executed for stock", order.order_id);
                    println!("#################");
                    println!();

                }
                // break;
            } 
            // else if stock.open <= order.entry_price && order.trade_position_type == TradeType::Short{
            //     let updated_order = order_manager.update_order( order, stock.open, redis_client, &order_collection).await;
            //     // println!("updated order: {:?} for index {}", updated_order, index);
            //     if updated_order.is_some(){
            //         let temp_order = updated_order.unwrap();
            //         order.is_trade_executed = temp_order.is_trade_executed;
            //         order.entry_price = temp_order.entry_price;
            //         order.order_executed_at = temp_order.order_executed_at;
            //         order.is_trade_executed = temp_order.is_trade_executed;
            //     }
                // break;
            }
    }

    //delete the closed orders from the shared order ledger
    for index in closed_order_indexes.iter(){
        shared_order_ledger.remove(*index);
    }
}

