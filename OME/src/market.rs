use rand::Rng;
use crate::helper::generate_random_string;

pub struct Stock {
    pub stock_id: String,
    pub stock_price: f32,
    pub stock_history: Vec<f64>,
}

pub struct Market {
    sentiment: MarketSentiment,
    sentiment_change_percent: f64,
    stocks: Vec<Stock>,
}

pub enum MarketSentiment {
    SuperBullish,
    Bullish,
    Neutral,
    Bearish,
    SuperBearish,
}
impl MarketSentiment {
    fn value(&self) -> f64 {
        match *self {
            MarketSentiment::SuperBullish => 3.5,
            MarketSentiment::Bullish => 1.25,
            MarketSentiment::Neutral => 0.0,
            MarketSentiment::Bearish => -1.25,
            MarketSentiment::SuperBearish => -3.5,
        }
    }
}
impl Market{
    pub fn tick(&mut self){
        for stock in self.stocks.iter_mut() {
            stock.generate_data();
        }
    }
    pub fn create_new_stock(&mut self){
        let mut rng = rand::thread_rng();
        self.stocks.push(Stock{stock_id:generate_random_string(), 
                                stock_price:rng.gen_range(10.0..401.0),
                                stock_history:Vec::new()})
    }
}

impl Stock{
    pub fn generate_data(&mut self) -> f64{
        let mut num = rand::thread_rng().gen_range(-100..100) as f64;
        num /= 1000.0;
        let ans = match self.stock_history.last(){
            Some(elem) => elem + elem * num,
            None => 100.0,
        };
        self.stock_history.push(ans.clone());
        return ans;
    }
    pub fn display_data(&mut self){

    }
}
