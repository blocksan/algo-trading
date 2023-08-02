use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::common::{enums::{AlgoTypes,TradeType}, redis_client::RedisClient, utils::{self, symbol_algo_type_formatter}, date_parser::new_current_date_time_in_desired_stock_datetime_format};
use super::trade_signal_keeper::TradeSignal;


// #[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub trade_position_type: TradeType,
    pub trade_algo_type: AlgoTypes,
    pub entry_price: f32,
    pub exit_price: f32,
    pub trade_sl: f32,
    pub trade_target: f32,
    pub is_trade_open: bool,
    pub qty: i32,
    pub total_price: f32,
    pub trade_taken_at: String,
    pub trade_closed_at: String,
    pub order_id: String,
    pub closing_profit: f32,
    pub is_profitable_trade: bool,
    
}

impl Order {
    pub fn new(
        symbol: String,
        trade_position_type: TradeType,
        trade_algo_type: AlgoTypes,
        entry_price: f32,
        exit_price: f32,
        trade_sl: f32,
        trade_target: f32,
        is_trade_open: bool,
        qty: i32,
        total_price: f32,
        trade_taken_at: String,
        trade_closed_at: String,
        order_id: String,
        closing_profit: f32,
        is_profitable_trade: bool,
    ) -> Order {
        Order {
            symbol,
            trade_position_type,
            trade_algo_type,
            entry_price,
            exit_price,
            trade_sl,
            trade_target,
            is_trade_open,
            qty,
            total_price,
            trade_taken_at,
            trade_closed_at,
            order_id,
            closing_profit,
            is_profitable_trade
        }
    }

    pub fn exit_trade(
        &mut self,
        exit_price: f32,
        trade_closed_at: String,
    ) -> (){
        self.exit_price = exit_price;
        self.trade_closed_at = trade_closed_at;
        self.is_trade_open = false;
        self.closing_profit = if self.trade_position_type == TradeType::Long {(self.exit_price - self.entry_price)*self.qty as f32} else {(self.entry_price - self.exit_price)*self.qty as f32};
        self.is_profitable_trade = self.closing_profit > 0.0;
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
pub struct OrderManager {
    orders: Vec<Order>,
}

impl OrderManager {
    pub fn new() -> OrderManager {
        OrderManager {
            orders: Vec::new(),
        }
    }

    pub async fn add_and_dispatch_order(&mut self, trade_signal: TradeSignal, order_collection: Collection<Order>) -> Option<Order>{
        let order_exists = OrderManager::check_if_order_exists(trade_signal.raw_stock.symbol.clone(), trade_signal.trade_algo_type.clone());
        
        if order_exists {
            println!("Order already exists for {} with algo type {}", trade_signal.raw_stock.symbol, trade_signal.trade_algo_type.to_string());
            None
        }else{
            let order = Order::new(
                trade_signal.raw_stock.symbol.clone(),
                trade_signal.trade_position_type,
                trade_signal.trade_algo_type.clone(),
                trade_signal.entry_price,
                0.0,
                trade_signal.trade_sl,
                trade_signal.trade_target,
                true,
                trade_signal.qty,
                trade_signal.total_price,
                trade_signal.trade_signal_requested_at,
                "".to_string(),
                symbol_algo_type_formatter(trade_signal.raw_stock.symbol.as_str(), trade_signal.trade_algo_type.to_string().as_str()),
                0.0,
                false,
            );

            println!("Order added to the order manager");

            //TODO:: add logic to call the Zerodha API to place the order
            match order_collection.insert_one(order.clone(), None).await{
                Ok(_) => {
                    println!("Order added to the database");
                },
                Err(e) => {
                    println!("Error while adding order to the database {}", e);
                }
            }
            self.orders.push(order.clone());
            Some(order)
        }
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    fn check_if_order_exists(symbol: String, trade_algo_type: AlgoTypes) -> bool {
        let mut order_exists = false;
        let redis_client = RedisClient::get_instance();

        let key = utils::symbol_algo_type_formatter(symbol.as_str(), trade_algo_type.to_string().as_str());

        match redis_client.lock().unwrap().get_data(key.as_str()){
            Ok(data) => {
                let parsed_data: i32 = data.parse().unwrap();
                if parsed_data > 0 {
                    order_exists = true;
                }
            },
            Err(e) => {
                println!("No order found for {} with Error {}", key, e);
                order_exists = false;
            }
        }
        order_exists
    }

    pub fn exit_and_update_order(&mut self, order: Order, exit_price: f32) -> (){
        let mut order_index = 0;
        for (index, order_in_orders) in self.orders.iter().enumerate() {
            if order_in_orders.order_id == order.order_id {
                order_index = index;
                break;
            }
        }

        let (closing_profit, is_profitable_trade) = OrderManager::calculate_profit(order.clone(), exit_price);

        let new_order = Order::new(
            order.symbol.clone(),
            order.trade_position_type.clone(),
            order.trade_algo_type.clone(),
            order.entry_price.clone(),
            exit_price,
            order.trade_sl.clone(),
            order.trade_target.clone(),
            false,
            order.qty.clone(),
            order.total_price.clone(),
            order.trade_taken_at.clone(),
            new_current_date_time_in_desired_stock_datetime_format(),
            order.order_id.clone(),
            closing_profit,
            is_profitable_trade,
        );

        let key = utils::symbol_algo_type_formatter(new_order.symbol.as_str(), new_order.trade_algo_type.to_string().as_str());
        
        self.orders[order_index] = new_order;

        let redis_client = RedisClient::get_instance();

        match redis_client.lock().unwrap().delete_data(key.as_str()){
            Ok(_) => {
                println!("Data deleted in Redis for key => {}", key);
            },
            Err(e) => {
                println!("Not able to delete {:?} with Error {:?}", key, e);
            }
        }


    }

    fn calculate_profit(order: Order, exit_price: f32) -> (f32, bool) {
        let profit = if order.trade_position_type == TradeType::Long {
            (exit_price - order.entry_price)*order.qty as f32
        }else{
            (order.entry_price - exit_price)*order.qty as f32
        };
        (profit, profit > 0.0)
    }
}