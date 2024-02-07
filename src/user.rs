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
    fn create_order(&self, price: f64, amount:i64, stock_id: String, order_type: OrderType, is_buy:bool) -> Order;
    fn search_market();
    fn view_order();
}
impl <'a> UserInterface for User<'a>{
    fn create_order(&self, price: f64, amount:i64, stock_id: String, order_type: OrderType, is_buy:bool) -> Order{
        let order = Order{user_id: self.user_id.clone(), is_fufilled: false, fufiller_id: None, price: price, amount: amount, timestamp:SystemTime::now(), order_type: order_type, is_buy: is_buy};
        return order;
    }
    fn search_market(){

    }
    fn view_order(){

    }
}


