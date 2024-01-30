mod order;
mod user;

use crate::order::Order;
use crate::user::User;

use std::time::SystemTime;

fn main() {
    let _user = User{name: String::from("kahshiuh"), balance:100f64, order:vec![], orderType: Or};
    let _me = Order{user_id: String::from("Hello"), is_fufilled: false, price: 100f64, timestamp: SystemTime::now()};
}
