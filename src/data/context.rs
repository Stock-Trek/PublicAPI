use std::collections::HashMap;

use crate::data::{market_data::market::Market, trading_pair::TradingPair};

pub struct StockTrekContext {
    markets: HashMap<TradingPair, Market>,
}

impl StockTrekContext {
    pub fn markets(&self) -> &HashMap<TradingPair, Market> {
        &self.markets
    }
}
