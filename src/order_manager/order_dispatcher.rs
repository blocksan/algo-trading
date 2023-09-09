use super::trade_signal_keeper::TradeSignal;
use crate::{
    common::{
        date_parser::new_current_date_time_in_desired_stock_datetime_format,
        enums::{AlgoTypes, TradeType},
        raw_stock::RawStock,
        redis_client::RedisClient,
        utils::{self},
    },
    order_manager::pnl_state::CurrentPnLState,
    user::user::User, config::mongodb_connection,
};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneOptions, UpdateOptions, FindOptions},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::Mutex, vec, str::FromStr};
use uuid::Uuid;
use futures::TryStreamExt;


#[derive(Deserialize, Debug)]
pub struct OrderBodyParams{
    pub start_trade_date: String,
    pub end_trade_date: Option<String>,
    pub user_id: String,
    pub trade_position_type: Option<TradeType>,
    pub trade_algo_type: Option<AlgoTypes>,
    pub is_trade_open: Option<bool>,
    pub is_profitable_trade: Option<bool>,
    pub total_price: Option<f32>,
    pub total_price_operator: Option<String>,
    pub symbol: Option<String>,
}

// #[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub trade_position_type: TradeType,
    pub trade_algo_type: AlgoTypes,
    pub entry_price: f32,
    pub entry_price_delta: f32,
    pub actual_entry_price: f32,
    pub exit_price: f32,
    pub exit_price_delta: f32,
    pub actual_exit_price: f32,
    pub trade_sl: f32,
    pub trade_target: f32,
    pub is_trade_open: bool,
    pub is_trade_executed: bool,
    pub qty: i32,
    pub total_price: f32,
    pub order_placed_at: String,
    pub order_executed_at: String,
    pub order_closed_at: String,
    pub order_id: String,
    pub order_cache_key: String,
    pub closing_profit: f32,
    pub is_profitable_trade: bool,
    pub algo_id: ObjectId,
    pub trade_signal_id: ObjectId,
    pub raw_stock: RawStock,
    pub user_id: ObjectId,
}

impl Order {
    pub fn new(
        symbol: String,
        trade_position_type: TradeType,
        trade_algo_type: AlgoTypes,
        entry_price: f32,
        entry_price_delta: f32,
        actual_entry_price: f32,
        exit_price: f32,
        exit_price_delta: f32,
        actual_exit_price: f32,
        trade_sl: f32,
        trade_target: f32,
        is_trade_open: bool,
        is_trade_executed: bool,
        qty: i32,
        total_price: f32,
        order_placed_at: String,
        order_executed_at: String,
        order_closed_at: String,
        order_id: String,
        order_cache_key: String,
        closing_profit: f32,
        is_profitable_trade: bool,
        algo_id: ObjectId,
        trade_signal_id: ObjectId,
        raw_stock: RawStock,
        user_id: ObjectId,
    ) -> Order {
        Order {
            symbol,
            trade_position_type,
            trade_algo_type,
            entry_price,
            entry_price_delta,
            actual_entry_price,
            exit_price,
            exit_price_delta,
            actual_exit_price,
            trade_sl,
            trade_target,
            is_trade_open,
            is_trade_executed,
            qty,
            total_price,
            order_placed_at,
            order_executed_at,
            order_closed_at,
            order_id,
            order_cache_key,
            closing_profit,
            is_profitable_trade,
            algo_id,
            trade_signal_id,
            raw_stock,
            user_id,
        }
    }

    pub fn exit_trade(&mut self, exit_price: f32, order_closed_at: String) -> () {
        self.exit_price = exit_price;
        self.order_closed_at = order_closed_at;
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
            "entry_price_delta": self.entry_price_delta.clone(),
            "actual_entry_price": self.actual_entry_price.clone(),
            "exit_price": self.exit_price.clone(),
            "exit_price_delta": self.exit_price_delta.clone(),
            "actual_exit_price": self.actual_exit_price.clone(),
            "trade_sl": self.trade_sl.clone(),
            "trade_target": self.trade_target.clone(),
            "is_trade_open": self.is_trade_open.clone(),
            "is_trade_executed": self.is_trade_executed.clone(),
            "qty": self.qty.clone(),
            "total_price": self.total_price.clone(),
            "order_placed_at": self.order_placed_at.clone(),
            "order_executed_at": self.order_executed_at.clone(),
            "order_closed_at": self.order_closed_at.clone(),
            "order_id": self.order_id.clone(),
            "order_cache_key": self.order_cache_key.clone(),
            "closing_profit": self.closing_profit.clone(),
            "is_profitable_trade": self.is_profitable_trade.clone(),
            "algo_id": self.algo_id.clone(),
            "trade_signal_id": self.trade_signal_id.clone(),
            "raw_stock": {
                "symbol": self.raw_stock.symbol.clone(),
                "date": self.raw_stock.date.clone(),
                "open": self.raw_stock.open,
                "high": self.raw_stock.high,
                "low": self.raw_stock.low,
                "close": self.raw_stock.close,
                "volume": self.raw_stock.volume,
                "market_time_frame": self.raw_stock.market_time_frame.to_string()
            },
            "user_id": self.user_id.clone(),
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
        user_collection: Collection<User>,
        shared_order_ledger: &mut Vec<Order>,
        current_pnl_state_collection: Collection<CurrentPnLState>,
    ) -> () {
        let users = User::get_all_active_users(user_collection).await;
        let mut dispatched_orders: Vec<Order> = vec![];
        for user in users {
            
            let (current_pnl_cache_key, current_pnl_cache_algo_types_key ) = CurrentPnLState::get_current_pnl_cache_key(
                trade_signal.raw_stock.date.as_str(),
                &user.id.to_string(),
            );

            let current_pnl_user_algo_types_options = CurrentPnLState::fetch_current_pnl_algo_types_of_user(
                current_pnl_cache_algo_types_key.as_str(),
            );

            if current_pnl_user_algo_types_options.is_none(){
                println!("No {} algo type found for user {}", trade_signal.trade_algo_type, user.id.to_string());
                continue;
            }

            let current_pnl_user_algo_types = current_pnl_user_algo_types_options.unwrap();
           
            if !current_pnl_user_algo_types.contains(&trade_signal.trade_algo_type) {
                println!("No tradeable {} algo types set by the user {}", trade_signal.trade_algo_type, user.id.to_string());
                continue;
            }


            let order_cache_key = utils::order_cache_key_formatter(
                &trade_signal.raw_stock.symbol,
                &trade_signal.trade_algo_type,
                &user.id.to_string(),
            );

            
            let order_exists = OrderManager::check_if_open_order_exists(
                order_cache_key.as_str(),
                redis_client,
                &order_collection,
            )
            .await;

            if order_exists {
                println!(
                    "Order already exists for {} with algo type {}",
                    trade_signal.raw_stock.symbol,
                    trade_signal.trade_algo_type.to_string()
                );
                ()
            } else {
                let order = Order::new(
                    trade_signal.raw_stock.symbol.clone(),
                    trade_signal.trade_position_type.clone(),
                    trade_signal.trade_algo_type.clone(),
                    trade_signal.entry_price,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    trade_signal.trade_sl,
                    trade_signal.trade_target,
                    true, //will be updated to false once the order is executed (order exited at exit price) on the exchange ie. Zerodha
                    false, //will be updated to true once the order is executed (order executed at entry price) on the exchange ie. Zerodha
                    trade_signal.qty,
                    trade_signal.total_price,
                    new_current_date_time_in_desired_stock_datetime_format(),
                    "".to_string(),
                    "".to_string(),
                    Uuid::new_v4().to_string(),
                    order_cache_key.clone(), //This is the order id which will be generated by the Zerodha API once the order is placed
                    0.0,
                    false,
                    trade_signal.algo_id.clone(),
                    trade_signal.id.clone(),
                    trade_signal.raw_stock.clone(),
                    user.id,
                );
                

                let (is_order_tradeable, current_pnl_state, not_eligible_trading_reason) =
                    OrderManager::check_if_order_is_tradeable(
                        redis_client,
                        current_pnl_cache_key.as_str(),
                        &order,
                        current_pnl_cache_algo_types_key.as_str(),
                        &current_pnl_state_collection
                    ).await;

                if !is_order_tradeable {
                    println!(
                    "Order for symbol {} is not tradeable due to {}",
                    trade_signal.raw_stock.symbol,
                    not_eligible_trading_reason,
                );
                    return;
                }

                if current_pnl_state.is_none() {
                    println!(
                    "Order is not tradeable for {} with algo type {} as current PnL state is not available",
                    trade_signal.raw_stock.symbol,
                    trade_signal.trade_algo_type.to_string()
                );
                    return;
                }

                //TODO:: add logic to call the Zerodha API to place the order
                match order_collection.insert_one(order.clone(), None).await {
                    Ok(_) => {
                        println!("Order added to the database");
                        match redis_client.lock().unwrap().set_data(
                            order.order_id.as_str(),
                            serde_json::to_string(&order.clone()).unwrap().as_str(),
                        ) {
                            Ok(_) => {
                                // println!("Order added in Redis for key => {}", order.order_id);
                            }
                            Err(e) => {
                                println!("Error while adding order in Redis => {:?}", e);
                            }
                        }

                        match redis_client
                            .lock()
                            .unwrap()
                            .set_data(order_cache_key.as_str(), "true")
                        {
                            Ok(_) => {
                                // println!("Order cache key added in Redis for key => {}", order_cache_key);
                            }
                            Err(e) => {
                                println!("Error while adding order cache key in Redis => {:?}", e);
                            }
                        }

                        let mut updated_current_pnl_state = current_pnl_state.unwrap();
                        updated_current_pnl_state.current_trade_count += 1;
                        updated_current_pnl_state.current_used_trade_capital += order.total_price;

                        match redis_client.lock().unwrap().set_data(
                            current_pnl_cache_key.as_str(),
                            serde_json::to_string(&updated_current_pnl_state)
                                .unwrap()
                                .as_str(),
                        ) {
                            Ok(_) => {
                                // println!("Order cache key added in Redis for key => {}", order_cache_key);
                            }
                            Err(e) => {
                                println!("Error while adding order cache key in Redis => {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error while adding order to the database {}", e);
                    }
                }
                // self.orders.push(order.clone());
                dispatched_orders.push(order.clone());
                // shared_order_ledger.push(order.clone());
                // ()
            }
        }
        shared_order_ledger.extend(dispatched_orders);
        drop(shared_order_ledger);
    }

    pub async fn fetch_orders(order_params: OrderBodyParams) -> Option<Vec<Order>> {
        let OrderBodyParams {
            start_trade_date,
            end_trade_date,
            user_id,
            trade_position_type,
            trade_algo_type,
            is_trade_open,
            is_profitable_trade,
            total_price,
            symbol,
            total_price_operator
        } = order_params;
        let mut filter = doc! {
            "user_id": ObjectId::from_str(user_id.as_str()).unwrap(),
            "order_placed_at": {
                "$gte": start_trade_date,
                "$lte": end_trade_date.unwrap_or(new_current_date_time_in_desired_stock_datetime_format())
            },
        }; 

        if trade_position_type.is_some() {
            filter.insert("trade_position_type", trade_position_type.unwrap().to_string());
        }

        if trade_algo_type.is_some() {
            filter.insert("trade_algo_type", trade_algo_type.unwrap().to_string());
        }

        if is_trade_open.is_some() {
            filter.insert("is_trade_open", is_trade_open.unwrap());
        }

        if is_profitable_trade.is_some() {
            filter.insert("is_profitable_trade", is_profitable_trade.unwrap());
        }

        if total_price.is_some() {
            if total_price_operator.is_some() && total_price_operator.unwrap() == "$gte"{
                filter.insert( "total_price", doc!{
                    "$gte": total_price
                });
            }else {
                filter.insert( "total_price", doc!{
                    "$lte": total_price
                });
            }
        }

        if symbol.is_some() {
            filter.insert("symbol", symbol.unwrap());
        }

        let options = FindOptions::builder().build();
        let order_collection = OrderManager::get_order_collection().await;

        // let cursor = pnl_configuration_collection.find(filter, options).await?.try_collect::<Vec<_>>().await?;
        let cursor = order_collection.find(filter, options).await;
        match cursor {
            Ok(_) => match cursor.unwrap().try_collect::<Vec<_>>().await {
                Ok(data) => {
                    // println!("Successfully fetched PnL configuration {:?}", pnl_configuration_found);
                    Some(data) 
                }
                Err(e) => {
                    println!("Cursor Error fetch_orders: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error fetch_orders: {}", e);
                None
            }
        }
    }

    async fn check_if_open_order_exists(
        cache_key: &str,
        redis_client: &Mutex<RedisClient>,
        order_collection: &Collection<Order>,
    ) -> bool {
        let mut order_exists = match redis_client.lock().unwrap().get_data(cache_key) {
            Ok(data) => {
                // println!("Order exists in Redis for key => {:?}", data);
                data.trim().parse().unwrap()
                // true
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

        let filter = doc! {"order_cache_key": cache_key.clone() };
        let options: FindOneOptions = FindOneOptions::builder().build();
        order_exists = match order_collection.find_one(filter, options).await {
            Ok(Some(order)) => {
                order.is_trade_open
            }
            Ok(None) => false,
            Err(e) => {
                println!("Error while fetching the data from MongoDB => {:?}", e);
                false
            }
        };
        if order_exists {
            println!("Existing order found for cache_key {}", cache_key);
        }
        order_exists
    }

    pub async fn close_executed_order(
        &mut self,
        order: &Order,
        exit_price: f32,
        redis_client: &Mutex<RedisClient>,
        order_collection: &Collection<Order>,
    ) -> Option<Order> {
        let (closing_profit, is_profitable_trade) =
            OrderManager::calculate_profit(order.clone(), exit_price);
        let exit_price_delta = exit_price - order.exit_price;

        let updated_order = Order::new(
            order.symbol.clone(),
            order.trade_position_type.clone(),
            order.trade_algo_type.clone(),
            order.entry_price.clone(),
            order.entry_price_delta.clone(),
            order.actual_entry_price.clone(),
            exit_price,
            exit_price_delta,
            exit_price,
            order.trade_sl.clone(),
            order.trade_target.clone(),
            false,
            true,
            order.qty,
            order.total_price,
            order.order_placed_at.clone(),
            order.order_executed_at.clone(),
            new_current_date_time_in_desired_stock_datetime_format(),
            order.order_id.clone(),
            order.order_cache_key.clone(),
            closing_profit,
            is_profitable_trade,
            order.algo_id.clone(),
            order.trade_signal_id.clone(),
            order.raw_stock.clone(),
            order.user_id,
        );
        let order_cache_key = order.order_cache_key.clone();

        // let redis_client = RedisClient::get_instance();
        let filter = doc! {"order_id": updated_order.order_id.clone() };
        let options = UpdateOptions::builder().build();
        let order_document = doc! {"$set":updated_order.clone().to_document()};
        match order_collection
            .update_one(filter, order_document, options)
            .await
        {
            Ok(_result) => {
                // println!("Successfully updated the order in MongoDB {:?}",result);
            }
            Err(e) => {
                println!(
                    "Error in MongoDB while updating the order: {:?} error {:?}",
                    order.order_id, e
                );
            }
        }

        match redis_client.lock().unwrap().set_data(
            updated_order.order_id.as_str(),
            serde_json::to_string(&updated_order.clone())
                .unwrap()
                .as_str(),
        ) {
            Ok(_) => {
                // println!("Order updated in Redis for order_id => {}", updated_order.order_id);
            }
            Err(e) => {
                println!(
                    "Not able to update/delete the order_id {:?} with Error {:?}",
                    order_cache_key, e
                );
            }
        }

        match redis_client
            .lock()
            .unwrap()
            .set_data(&order_cache_key, "false")
        {
            Ok(_) => {
                // println!("Order cache updated in Redis for order_cache_key => {}", order_cache_key);
            }
            Err(e) => {
                println!(
                    "Not able to update/delete the order_cache_key {:?} with Error {:?}",
                    order_cache_key, e
                );
            }
        }

        Some(updated_order)
        //calculate current PnL
    }

    pub async fn mark_order_executed(
        &mut self,
        order: &Order,
        actual_entry_price: f32,
        redis_client: &Mutex<RedisClient>,
        order_collection: &Collection<Order>
    ) -> Option<Order> {
        let entry_price_delta = actual_entry_price - order.entry_price;
        let executed_order = Order::new(
            order.symbol.clone(),
            order.trade_position_type.clone(),
            order.trade_algo_type.clone(),
            order.entry_price.clone(),
            entry_price_delta,
            actual_entry_price,
            order.exit_price.clone(),
            order.exit_price_delta.clone(),
            order.actual_exit_price.clone(),
            order.trade_sl.clone(),
            order.trade_target.clone(),
            order.is_trade_open,
            true,
            order.qty,
            order.total_price,
            order.order_placed_at.clone(),
            new_current_date_time_in_desired_stock_datetime_format(),
            order.order_closed_at.clone(),
            order.order_id.clone(),
            order.order_cache_key.clone(),
            order.closing_profit.clone(),
            order.is_profitable_trade,
            order.algo_id.clone(),
            order.trade_signal_id.clone(),
            order.raw_stock.clone(),
            order.user_id,
        );

        let order_cache_key = order.order_cache_key.clone();

        // let redis_client = RedisClient::get_instance();
        let filter = doc! {"order_id": executed_order.order_id.clone() };
        let options = UpdateOptions::builder().build();
        let order_document = doc! {"$set":executed_order.clone().to_document()};
        match order_collection
            .update_one(filter, order_document, options)
            .await
        {
            Ok(_result) => {
                // println!("Successfully updated the order in MongoDB {:?}",result);
            }
            Err(e) => {
                println!(
                    "Error in MongoDB while executing the order: {:?} error {:?}",
                    order.order_id, e
                );
            }
        }

        match redis_client.lock().unwrap().set_data(
            executed_order.order_id.as_str(),
            serde_json::to_string(&executed_order.clone())
                .unwrap()
                .as_str(),
        ) {
            Ok(_) => {
                // println!("Order executed in Redis for order_id => {}", executed_order.order_id);
            }
            Err(e) => {
                println!(
                    "Not able to execute the order_id {:?} with Error {:?}",
                    order_cache_key, e
                );
            }
        }

        // match redis_client.lock().unwrap().set_data(&order_cache_key, "true") {
        //     Ok(_) => {
        //         // println!("Order cache updated in Redis for order_cache_key => {}", order_cache_key);
        //     }
        //     Err(e) => {
        //         println!("Not able to execute the order_cache_key {:?} with Error {:?}", order_cache_key, e);
        //     }
        // }

        Some(executed_order)
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

    async fn check_if_order_is_tradeable(
        redis_client: &Mutex<RedisClient>,
        current_pnl_cache_key: &str,
        order: &Order,
        current_pnl_cache_algo_types_key: &str,
        current_pnl_state_collection: &Collection<CurrentPnLState>,
    ) -> (bool, Option<CurrentPnLState>, String) {
        let current_pnl = match redis_client.lock().unwrap().get_data(current_pnl_cache_key) {
            Ok(data) => {
                let formatted_current_pnl =
                    serde_json::from_str::<CurrentPnLState>(data.as_str()).unwrap();
                // println!("Current PnL updated => {:?}", formatted_current_pnl.clone());
                Some(formatted_current_pnl)
            }
            Err(e) => {
                println!("No cache PnL found with Error {:?}", e);

                None
            }
        };

        if current_pnl.is_none() {
            let not_eligible_trading_reason = "No current PnL found".to_string();
            println!(
                "{} the current_pnl_cache_key => {:?}",not_eligible_trading_reason,
                current_pnl_cache_key
            );
            return (false, None, not_eligible_trading_reason);
        } else {
            let mut current_pnl = current_pnl.unwrap();
            if current_pnl.is_eligible_for_trading
                && (current_pnl.current_used_trade_capital + order.total_price)
                    < current_pnl.max_trade_capital
            {
                return (true, Some(current_pnl),"".to_string());
            } else if !current_pnl.is_eligible_for_trading {
                (false, None, current_pnl.not_eligible_trading_reason)
            }
            else if (current_pnl.current_used_trade_capital+order.total_price) > current_pnl.max_trade_capital {
                let not_eligible_trading_reason = "Max trade capital limit reached".to_string();
                current_pnl.is_eligible_for_trading = false;
                current_pnl.not_eligible_trading_reason = not_eligible_trading_reason.clone();
                current_pnl.push_current_pnl_state_to_redis_mongo(
                    current_pnl_cache_key,
                    current_pnl_cache_algo_types_key,
                    current_pnl_state_collection
                ).await;

                (false, None, not_eligible_trading_reason)
            }else{
                (false, None, "Unknown reason".to_string())
            }
        }
    }

    pub async fn get_order_collection() -> Collection<Order>{
        let db = mongodb_connection::fetch_db_connection().await;
        let order_collection_name = "orders";
        let order_collection = db.collection::<Order>(order_collection_name);
        return order_collection
    }
}
