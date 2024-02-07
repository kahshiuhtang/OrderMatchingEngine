use std::time::SystemTime;
pub enum OrderType{
    Market,
}
pub struct Order {
    pub user_id: String,
    pub order_id: String, 
    pub is_fufilled: bool,
    pub fufiller_id: Option<String>,
    pub price: f64,
    pub amount: i64,
    pub timestamp: SystemTime,
    pub order_type: OrderType,
    pub is_buy: bool,
}

