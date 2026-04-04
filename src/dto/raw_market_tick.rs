use crate::dto::raw_market_quote::RawMarketQuote;
use crate::prelude::TimestampMillis;

#[derive(Debug, Clone)]
pub struct RawMarketTick {
    timestamp_millis: TimestampMillis,
    bid: RawMarketQuote,
    ask: RawMarketQuote,
    last: RawMarketQuote,
}

impl RawMarketTick {
    pub fn timestamp_millis(&self) -> TimestampMillis {
        self.timestamp_millis
    }
    pub fn bid(&self) -> &RawMarketQuote {
        &self.bid
    }
    pub fn ask(&self) -> &RawMarketQuote {
        &self.ask
    }
    pub fn last(&self) -> &RawMarketQuote {
        &self.last
    }
}
