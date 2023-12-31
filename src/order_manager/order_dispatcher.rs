use super::trade_signal_keeper::TradeSignal;
use crate::common::{
    date_parser::new_current_date_time_in_desired_stock_datetime_format,
    enums::{AlgoTypes, TradeType},
    redis_client::RedisClient,
    utils::{self, order_cache_key_formatter},
};
use mongodb::{bson::{doc, Document}, options::{FindOneOptions, UpdateOptions}, Collection};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::{Mutex, Arc}};

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
            is_profitable_trade,
        }
    }

    pub fn exit_trade(&mut self, exit_price: f32, trade_closed_at: String) -> () {
        self.exit_price = exit_price;
        self.trade_closed_at = trade_closed_at;
        self.is_trade_open = false;
        self.closing_profit = if self.trade_position_type == TradeType::Long {
            (self.exit_price - self.entry_price) * self.qty as f32
        } else {
            (self.entry_price - self.exit_price) * self.qty as f32
        };
        self.is_profitable_trade = self.closing_profit > 0.0;
    }

    pub fn to_document(&self) -> Document {
        doc! {
            "symbol": self.symbol.clone(),
            "trade_position_type": self.trade_position_type.clone().to_string(),
            "trade_algo_type": self.trade_algo_type.clone().to_string(),
            "entry_price": self.entry_price.clone(),
            "exit_price": self.exit_price.clone(),
            "trade_sl": self.trade_sl.clone(),
            "trade_target": self.trade_target.clone(),
            "is_trade_open": self.is_trade_open.clone(),
            "qty": self.qty.clone(),
            "total_price": self.total_price.clone(),
            "trade_taken_at": self.trade_taken_at.clone(),
            "trade_closed_at": self.trade_closed_at.clone(),
            "order_id": self.order_id.clone(),
            "closing_profit": self.closing_profit.clone(),
            "is_profitable_trade": self.is_profitable_trade.clone(),
        }
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
        OrderManager { orders: Vec::new() }
    }

    pub async fn check_and_dispatch_order(
        &mut self,
        trade_signal: TradeSignal,
        redis_client: &Mutex<RedisClient>,
        order_collection: Collection<Order>,
        shared_order_ledger: Arc<Mutex<Vec<Order>>>
    ) -> () {
        let order_cache_key = utils::order_cache_key_formatter(
            &trade_signal.raw_stock.symbol,
            &trade_signal.trade_algo_type,
        );
        let order_exists = OrderManager::check_if_order_exists(
            order_cache_key.as_str(),
            redis_client,
            &order_collection,
        )
        .await;

        if order_exists {
            // println!(
            //     "Order already exists for {} with algo type {}",
            //     trade_signal.raw_stock.symbol,
            //     trade_signal.trade_algo_type.to_string()
            // );
            ()
        } else {
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
                order_cache_key_formatter(
                    trade_signal.raw_stock.symbol.as_str(),
                    &trade_signal.trade_algo_type,
                ), //This is the order id which will be generated by the Zerodha API once the order is placed
                0.0,
                false,
            );

            //TODO:: add logic to call the Zerodha API to place the order
            match order_collection.insert_one(order.clone(), None).await {
                Ok(_) => {
                    println!("Order added to the database");
                    match redis_client
                        .lock()
                        .unwrap()
                        .set_data(order_cache_key.as_str(), serde_json::to_string(&order.clone()).unwrap().as_str())
                    {
                        Ok(_) => {
                            println!("Order added in Redis for key => {}", order_cache_key);
                        }
                        Err(e) => {
                            println!("Error while adding order in Redis => {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error while adding order to the database {}", e);
                }
            }
            // self.orders.push(order.clone());
            shared_order_ledger.lock().unwrap().push(order.clone());
            ()
        }
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    async fn check_if_order_exists(
        cache_key: &str,
        redis_client: &Mutex<RedisClient>,
        order_collection: &Collection<Order>,
    ) -> bool {
        let mut order_exists = match redis_client.lock().unwrap().get_data(cache_key) {
            Ok(data) => {
                true
            }
            Err(e) => {
                println!(
                    "No order found in REDIS for {} with Error {:?}",
                    cache_key, e
                );

                false
            }
        };

        if order_exists {
            // println!("Order exists in Redis for key => {}", cache_key);
            return order_exists;
        }

        let filter = doc! {"cache_key": cache_key.clone() };
        let options: FindOneOptions = FindOneOptions::builder().build();
        order_exists = match order_collection.find_one(filter, options).await {
            Ok(Some(order)) => {
                println!("Existing order found for {} => {:?}", cache_key, order);
                true
            }
            Ok(None) => false,
            Err(e) => {
                println!("Error while fetching the data from MongoDB => {:?}", e);
                false
            }
        };
        order_exists
    }

    pub async fn exit_and_update_order(&mut self, order: &Order, exit_price: f32, redis_client: &Mutex<RedisClient>, order_collection: &Collection<Order>) -> Option<Order> {
        // for (index, order_in_orders) in self.orders.iter().enumerate() {
        //     if order_in_orders.order_id == order.order_id {
        //         order_index = index;
        //         break;
        //     }
        // }

        //TODO: call the Zerodha API to exit the order

        let (closing_profit, is_profitable_trade) =
            OrderManager::calculate_profit(order.clone(), exit_price);

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

        let order_cache_key =
            utils::order_cache_key_formatter(new_order.symbol.as_str(), &new_order.trade_algo_type);


        // let redis_client = RedisClient::get_instance();
        let filter = doc! {"order_id": order_cache_key.clone() };
        let options = UpdateOptions::builder().build();
        let order_document = doc!{"$set":new_order.clone().to_document()};
        match order_collection.update_one(
                filter,
                order_document,            
                options,
            ).await {
                Ok(result) => {
                    println!("Successfully updated the order in MongoDB {:?}",result);
                },
                Err(e) => {
                    println!("Error in MongoDB while updating the order: {:?} error {:?}", order.order_id,e);
                }
        }
            

        match redis_client.lock().unwrap().set_data(order_cache_key.as_str(), serde_json::to_string(&new_order.clone()).unwrap().as_str()) {
            Ok(_) => {
                println!("Order updated in Redis for order_id => {}", order_cache_key);
            }
            Err(e) => {
                println!("Not able to update/delete the order_id {:?} with Error {:?}", order_cache_key, e);
            }
        }

        Some(new_order)
        //calculate current PnL
    }

    fn calculate_profit(order: Order, exit_price: f32) -> (f32, bool) {
        let profit = if order.trade_position_type == TradeType::Long {
            (exit_price - order.entry_price) * order.qty as f32
        } else {
            (order.entry_price - exit_price) * order.qty as f32
        };
        (profit, profit > 0.0)
    }
}
