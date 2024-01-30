use crate::order::Order;
pub struct User<'a>{
    pub name: String,
    pub balance: f64,
    pub order: Vec<&'a Order>,
}

trait UserInterface {
    fn createOrder();
}