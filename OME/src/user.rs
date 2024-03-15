use crate::helper::generate_random_string;
use crate::market::Stock;
use crate::order::{Order, OrderType};

use std::time::SystemTime;

pub struct User<'a> {
    pub name: String,
    pub user_id: String,
    pub balance: f64,
    pub order: Vec<&'a Order>,
}

trait UserInterface {
    fn create_order(
        &self,
        price: f64,
        amount: i64,
        stock_id: String,
        order_type: OrderType,
        is_buy: bool,
    ) -> Order;
    fn search_market(&self, stock_id: String);
    fn view_order();
}
impl<'a> UserInterface for User<'a> {
    fn create_order(
        &self,
        price: f64,
        amount: i64,
        stock_id: String,
        order_type: OrderType,
        is_buy: bool,
    ) -> Order {
        let order = Order {
            stock_id,
            order_id: generate_random_string(),
            user_id: self.user_id.clone(),
            is_fulfilled: false,
            fulfiller_id: None,
            price,
            amount,
            timestamp: SystemTime::now(),
            order_type,
            is_buy,
        };
        return order;
    }
    fn search_market(&self, stock_id: String) {}
    fn view_order() {}
}
