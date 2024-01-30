use std::time::SystemTime;

pub struct OrderTypeContainer{
    orderType: OrderType,
    isBuy: bool,
}
pub enum OrderType{
    Market,
    Limit
}
pub struct Order{
    pub user_id: String,
    pub is_fufilled: bool,
    pub price: f64,
    pub timestamp: SystemTime,
    orderType: OrderTypeContainer,
}