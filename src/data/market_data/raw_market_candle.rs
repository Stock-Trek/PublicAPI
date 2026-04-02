use crate::data::{market_data::raw_market_ohlcv::RawMarketOhlcv, timestamp::TimestampMillis};

#[derive(Debug, Clone)]
pub struct RawMarketCandle {
    start_time_millis_inc: TimestampMillis,
    end_time_millis_exc: TimestampMillis,
    duration_millis: TimestampMillis,
    is_candle_closed: bool,
    ohlcv: RawMarketOhlcv,
    trade_count: u64,
}

impl RawMarketCandle {
    pub fn start_time_millis_inc(&self) -> TimestampMillis {
        self.start_time_millis_inc
    }
    pub fn end_time_millis_exc(&self) -> TimestampMillis {
        self.end_time_millis_exc
    }
    pub fn duration_millis(&self) -> TimestampMillis {
        self.duration_millis
    }
    pub fn is_candle_closed(&self) -> bool {
        self.is_candle_closed
    }
    pub fn ohlcv(&self) -> &RawMarketOhlcv {
        &self.ohlcv
    }
    pub fn trade_count(&self) -> u64 {
        self.trade_count
    }
}
