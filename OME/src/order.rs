use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::SystemTime;

use crate::market::Stock;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrderType {
    Market,
    Limit,
}
#[derive(Debug, PartialOrd)]
pub struct Order {
    pub order_id: String,
    pub stock_id: String,
    pub user_id: String,
    pub is_fulfilled: bool,
    pub fulfiller_id: Option<String>,
    pub price: f64,
    pub amount: i64,
    pub timestamp: SystemTime,
    pub order_type: OrderType,
    pub is_buy: bool,
}
impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        return self.order_id == other.order_id;
    }
}
impl Eq for Order {}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price > other.price {
            return Ordering::Less;
        }
        if self.price < other.price {
            return Ordering::Greater;
        }
        if self.amount > other.amount {
            return Ordering::Less;
        }
        if self.amount < other.amount {
            return Ordering::Greater;
        }
        if self.timestamp > other.timestamp {
            return Ordering::Less;
        }
        if self.timestamp < other.timestamp {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}
pub struct StockOrders {
    pub sell_orders: BinaryHeap<Order>,
    pub buy_orders: BinaryHeap<Order>,
}
pub struct OrderBook {
    pub orders_to_fufill: Vec<String>,
    pub timestamp: i64,
    pub order_mappings: HashMap<String, StockOrders>,
    pub stocks_mappings: HashMap<String, Stock>,
}

impl OrderBook {
    pub fn handle_order(&mut self, order: Order) {
        if order.order_type == OrderType::Limit {
            self.insert_order(order)
        }
    }
    fn insert_order(&mut self, order: Order) {
        if order.is_buy {
            let stock_order_map = self.order_mappings.get(&order.stock_id);
            match stock_order_map {
                Some(map) => {}
                None => {
                    let mut sell_orders = BinaryHeap::new();
                    let mut buy_orders = BinaryHeap::new();
                    match order.is_buy {
                        true => buy_orders.push(order),
                        false => sell_orders.push(order),
                    }
                    self.order_mappings.insert(
                        String::from(order.stock_id.borrow()),
                        StockOrders {
                            sell_orders,
                            buy_orders,
                        },
                    );
                }
            }
        } else {
        }
    }
    pub fn find_matches() {}
    pub fn ticker() {}
}
