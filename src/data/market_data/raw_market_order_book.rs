use crate::data::market_data::raw_market_quote::RawMarketQuote;

#[derive(Debug, Clone)]
pub struct RawMarketOrderBook {
    bids: Vec<RawMarketQuote>,
    asks: Vec<RawMarketQuote>,
}

impl RawMarketOrderBook {
    pub fn bids(&self) -> &Vec<RawMarketQuote> {
        &self.bids
    }
    pub fn asks(&self) -> &Vec<RawMarketQuote> {
        &self.asks
    }
}
