use crate::{market_data::market::Market, prelude::TradingPair};
use std::collections::HashMap;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
#[non_exhaustive]
#[repr(u8)]
pub enum ExchangeName {
    Binance,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Exchange {
    markets: HashMap<TradingPair, Market>,
}

impl Exchange {
    pub fn new(markets: HashMap<TradingPair, Market>) -> Self {
        Self { markets }
    }
    pub fn markets(&self) -> &HashMap<TradingPair, Market> {
        &self.markets
    }
}
