use crate::{
    exchange::{Exchange, ExchangeName},
    statistics::stats::Stats,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct StockTrekContext {
    exchanges: HashMap<ExchangeName, Exchange>,
    pub stats: Stats,
}

impl StockTrekContext {
    pub fn exchanges(&self) -> &HashMap<ExchangeName, Exchange> {
        &self.exchanges
    }
}
