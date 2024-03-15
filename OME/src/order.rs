use std::cmp::Ordering;
use std::cmp::Reverse;
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
    pub amount_fulfilled: i64,
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
        let stock_order_map = self.order_mappings.get_mut(&order.stock_id);
        match stock_order_map {
            Some(stock_order) => {
                if order.is_buy {
                    stock_order.buy_orders.push(order)
                } else {
                    stock_order.sell_orders.push(order)
                }
            }
            None => {
                let stock_id_copy = String::from(&order.stock_id);
                let mut sell_orders = BinaryHeap::new();
                let mut buy_orders = BinaryHeap::new();
                match order.is_buy {
                    true => buy_orders.push(order),
                    false => sell_orders.push(order),
                }
                self.order_mappings.insert(
                    stock_id_copy,
                    StockOrders {
                        sell_orders,
                        buy_orders,
                    },
                );
            }
        }
    }
    pub fn find_matches(&mut self) {
        for (key, value) in &mut self.order_mappings {
            // Want to take the highest buys and the lowest sells
            // If they overlap, then we can match them
            let mut highest_buy_orders: Vec<Order> = Vec::new();
            let mut lowest_sell_orders: Vec<Order> = Vec::new();
            let mut buy_price = -1.0;
            let mut total_buy_amount = 0;
            let mut sell_price = -1.0;
            let mut total_sell_amount = 0;
            while let Some(buy_order) = value.buy_orders.pop() {
                if buy_price == -1f64 {
                    buy_price = buy_order.price;
                    total_buy_amount += buy_order.amount;
                    highest_buy_orders.push(buy_order);
                } else if buy_order.price == buy_price {
                    total_buy_amount += buy_order.amount;
                    highest_buy_orders.push(buy_order);
                } else {
                    break;
                }
            }
            while let Some(sell_order) = value.sell_orders.pop() {
                if sell_price == -1f64 {
                    sell_price = sell_order.price;
                    total_sell_amount += sell_order.amount;
                    lowest_sell_orders.push(sell_order);
                } else if sell_order.price == sell_price {
                    total_sell_amount += sell_order.amount;
                    lowest_sell_orders.push(sell_order);
                } else {
                    break;
                }
            }
            if buy_price > sell_price {
            } else {
                while let Some(buy_order) = highest_buy_orders.pop() {
                    value.buy_orders.push(buy_order)
                }
                while let Some(sell_order) = lowest_sell_orders.pop() {
                    value.sell_orders.push(sell_order)
                }
            }
        }
    }
    pub fn ticker() {}
}
