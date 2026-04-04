use crate::{
    dto::raw_market_candle::RawMarketCandle, market_data::extract::dec_to_f64,
    rolling_window::RollingWindow,
};
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct MarketRollingWindow {
    exact: HashMap<RollingWindow, RawMarketCandle>,
    ohlcv: OnceLock<HashMap<RollingWindow, OnceLock<Vec<f64>>>>,
}

impl MarketRollingWindow {
    pub fn new(exact: HashMap<RollingWindow, RawMarketCandle>) -> Self {
        Self {
            exact,
            ohlcv: OnceLock::new(),
        }
    }
    pub fn exact(&self) -> &HashMap<RollingWindow, RawMarketCandle> {
        &self.exact
    }
    pub fn ohlcv(&self, window: RollingWindow) -> &Vec<f64> {
        self.ohlcv
            .get_or_init(|| self.new_ohlcv_map())
            .get(&window)
            .unwrap()
            .get_or_init(|| self.ohlcv_values(window))
    }

    fn ohlcv_values(&self, window: RollingWindow) -> Vec<f64> {
        let ohlcv = self.exact.get(&window).unwrap().ohlcv();
        vec![
            dec_to_f64(ohlcv.open()),
            dec_to_f64(ohlcv.high()),
            dec_to_f64(ohlcv.low()),
            dec_to_f64(ohlcv.close()),
            dec_to_f64(ohlcv.volume()),
            dec_to_f64(ohlcv.quote_volume()),
            dec_to_f64(ohlcv.vwap()),
        ]
    }
    fn new_ohlcv_map(&self) -> HashMap<RollingWindow, OnceLock<Vec<f64>>> {
        RollingWindow::iter()
            .map(|window| (window, OnceLock::new()))
            .collect()
    }
}
