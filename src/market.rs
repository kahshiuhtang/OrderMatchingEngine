use rand::Rng;

pub struct Stock {
    pub stock_id: i32,
    pub stock_price: f32,
    pub open: f32,
    pub close: f32,
    pub stock_history: Vec<f64>,
}

pub struct Market {
    sentiment: MarketSentiment,
    sentiment_change_percent: f64,
    stocks: [Stock],

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
