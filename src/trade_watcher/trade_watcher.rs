use crate::{order_manager::order_dispatcher::OrderManager, common::{raw_stock::RawStock, enums::TradeType, date_parser::new_current_date_time_in_desired_stock_datetime_format}};


pub fn check_for_exit_opportunity(order_manager: &mut OrderManager, stock: RawStock){
    let orders = order_manager.get_orders().clone();
    let mut exit_price = 0.0;
    for order in orders{
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
                order_manager.exit_and_update_order(order, exit_price);
                break;
            }
        }
    }
}