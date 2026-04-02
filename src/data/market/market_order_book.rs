use crate::data::market::{extract::vec_quote_to_f64, raw_market_order_book::RawMarketOrderBook};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct MarketOrderBook {
    exact: RawMarketOrderBook,
    bids: OnceLock<Vec<(f64, f64)>>,
    asks: OnceLock<Vec<(f64, f64)>>,
}

impl MarketOrderBook {
    pub fn new(exact: RawMarketOrderBook) -> Self {
        Self {
            exact,
            bids: OnceLock::new(),
            asks: OnceLock::new(),
        }
    }
    pub fn exact(&self) -> &RawMarketOrderBook {
        &self.exact
    }
    pub fn bids(&self) -> &Vec<(f64, f64)> {
        &self
            .bids
            .get_or_init(|| vec_quote_to_f64(self.exact.bids()))
    }
    pub fn asks(&self) -> &Vec<(f64, f64)> {
        &self
            .asks
            .get_or_init(|| vec_quote_to_f64(self.exact.asks()))
    }
}
