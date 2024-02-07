pub struct Stock {
    stock_id: i32,
    stock_price: f32,
    open: f32,
    close: f32,
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
