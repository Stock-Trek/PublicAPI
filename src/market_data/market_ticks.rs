use crate::{
    dto::{raw_market_quote::RawMarketQuote, raw_market_tick::RawMarketTick},
    market_data::extract::dec_to_f64,
    prelude::TimestampMillis,
};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct MarketTicks {
    exact: Vec<RawMarketTick>,
    bids: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
    asks: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
    lasts: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
}

impl MarketTicks {
    pub fn new(exact: Vec<RawMarketTick>) -> Self {
        Self {
            exact,
            bids: OnceLock::new(),
            asks: OnceLock::new(),
            lasts: OnceLock::new(),
        }
    }
    pub fn exact(&self) -> &Vec<RawMarketTick> {
        &self.exact
    }
    pub fn bids(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.bids.get_or_init(|| self.values(|tick| tick.bid()))
    }
    pub fn asks(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.asks.get_or_init(|| self.values(|tick| tick.ask()))
    }
    pub fn lasts(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.lasts.get_or_init(|| self.values(|tick| tick.last()))
    }

    fn values(
        &self,
        to_quote: impl Fn(&RawMarketTick) -> &RawMarketQuote,
    ) -> Vec<(TimestampMillis, f64, f64)> {
        self.exact
            .iter()
            .map(|tick| to_tick_tuple(tick, &to_quote))
            .collect()
    }
}

fn to_tick_tuple(
    tick: &RawMarketTick,
    to_quote: impl Fn(&RawMarketTick) -> &RawMarketQuote,
) -> (TimestampMillis, f64, f64) {
    let timestamp = tick.timestamp_millis();
    let quote = to_quote(tick);
    let price = dec_to_f64(quote.price());
    let quantity = dec_to_f64(quote.quantity());
    (timestamp, price, quantity)
}
