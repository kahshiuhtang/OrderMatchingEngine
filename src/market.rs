pub struct Stock {
    stock_id: i32,
    stock_price: f32,
    open: f32,
    close: f32,
}

pub struct Market {
    sentiment: MarketSentiment,
    stocks: [Stock],
}

pub enum MarketSentiment {
    SUPER_BULLISH,
    BULLISH,
    NEUTRAL,
    BEARISH,
    SUPER_BEARISH,
}
impl MarketSentiment {
    fn value(&self) -> f64 {
        match *self {
            MarketSentiment::SUPER_BULLISH => 3.5,
            MarketSentiment::BULLISH => 1.25,
            MarketSentiment::NEUTRAL => 0.0,
            MarketSentiment::BEARISH => -1.25,
            MarketSentiment::SUPER_BEARISH => -3.5,
        }
    }
}
