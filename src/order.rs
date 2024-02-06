use std::time::SystemTime;

pub struct OrderTypeContainer {
    pub order_type: OrderType,
    pub is_buy: bool,
}

pub enum OrderType {
    Market,
    Limit,
}
pub struct Order {
    pub user_id: String,
    pub is_fufilled: bool,
    pub price: f64,
    pub timestamp: SystemTime,
    pub order_type: OrderTypeContainer,
}
