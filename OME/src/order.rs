use std::time::SystemTime;
pub enum OrderType{
    Market,
}
pub struct Order {
    pub order_id: String, 
    pub user_id: String,
    pub is_fufilled: bool,
    pub fufiller_id: Option<String>,
    pub price: f64,
    pub amount: i64,
    pub timestamp: SystemTime,
    pub order_type: OrderType,
    pub is_buy: bool,
}

pub struct OrderBook{
    pub orders_to_fufill: Vec<String>,
    pub timestamp: i64,
}

impl OrderBook{
    pub fn send_order(){

    }
    pub fn find_matches(){

    }
    pub fn match_order(&mut order1, &mut order2){

    }
}

