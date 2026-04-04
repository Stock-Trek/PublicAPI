use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct RawMarketOhlcv {
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
    quote_volume: Decimal,
    vwap: Decimal,
}

impl RawMarketOhlcv {
    pub fn open(&self) -> Decimal {
        self.open
    }
    pub fn high(&self) -> Decimal {
        self.high
    }
    pub fn low(&self) -> Decimal {
        self.low
    }
    pub fn close(&self) -> Decimal {
        self.close
    }
    pub fn volume(&self) -> Decimal {
        self.volume
    }
    pub fn quote_volume(&self) -> Decimal {
        self.quote_volume
    }
    pub fn vwap(&self) -> Decimal {
        self.vwap
    }
}
