use crate::data::market_data::{
    extract::dec_to_f64, market_window::AlignedWindow, raw_market_candle::RawMarketCandle,
    raw_market_ohlcv::RawMarketOhlcv,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct MarketAlignedWindow {
    exact: HashMap<AlignedWindow, Vec<RawMarketCandle>>,
    opens: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    highs: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    lows: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    closes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    quote_volumes: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
    vwaps: OnceLock<HashMap<AlignedWindow, OnceLock<Vec<f64>>>>,
}

impl MarketAlignedWindow {
    pub fn new(exact: HashMap<AlignedWindow, Vec<RawMarketCandle>>) -> Self {
        Self {
            exact,
            opens: OnceLock::new(),
            highs: OnceLock::new(),
            lows: OnceLock::new(),
            closes: OnceLock::new(),
            volumes: OnceLock::new(),
            quote_volumes: OnceLock::new(),
            vwaps: OnceLock::new(),
        }
    }
    pub fn exact(&self) -> &HashMap<AlignedWindow, Vec<RawMarketCandle>> {
        &self.exact
    }
    pub fn opens(&self, window: AlignedWindow) -> &Vec<f64> {
        self.opens
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.open()))
    }
    pub fn highs(&self, window: AlignedWindow) -> &Vec<f64> {
        self.highs
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.high()))
    }
    pub fn lows(&self, window: AlignedWindow) -> &Vec<f64> {
        self.lows
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.low()))
    }
    pub fn closes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.closes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.close()))
    }
    pub fn volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.volume()))
    }
    pub fn quote_volumes(&self, window: AlignedWindow) -> &Vec<f64> {
        self.quote_volumes
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.quote_volume()))
    }
    pub fn vwaps(&self, window: AlignedWindow) -> &Vec<f64> {
        self.vwaps
            .get_or_init(windows_map)
            .get(&window)
            .unwrap()
            .get_or_init(|| self.aligned_values(window, |ohlcv| ohlcv.vwap()))
    }

    fn aligned_values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&RawMarketOhlcv) -> Decimal,
    ) -> Vec<f64> {
        self.exact
            .get(&window)
            .unwrap()
            .iter()
            .map(|candle| dec_to_f64(to_value(candle.ohlcv())))
            .collect()
    }
}

fn windows_map() -> HashMap<AlignedWindow, OnceLock<Vec<f64>>> {
    AlignedWindow::iter()
        .map(|window| (window, OnceLock::new()))
        .collect()
}
