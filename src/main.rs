use crate::order::Order;

use std::time::SystemTime;

mod order;
fn main() {
    println!("Hello, world!");
    let _me = Order{price: 100f64, timestamp: SystemTime::now()};
}
