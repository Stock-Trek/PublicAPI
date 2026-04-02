#[cfg(feature = "python")]
use {
    crate::bindings::python::data::market::{
        py_market::PyMarket,
        py_market_aligned_window::PyMarketAlignedWindow,
        py_market_order_book::PyMarketOrderBook,
        py_market_rolling_window::PyMarketRollingWindow,
        py_market_ticks::PyMarketTicks,
        py_market_window::{PyAlignedWindow, PyRollingWindow},
        py_raw_market_candle::PyRawMarketCandle,
        py_raw_market_ohlcv::PyRawMarketOhlcv,
        py_raw_market_order_book::PyRawMarketOrderBook,
        py_raw_market_quote::PyRawMarketQuote,
        py_raw_market_tick::PyRawMarketTick,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "market")?;
    module.add_class::<PyMarketAlignedWindow>()?;
    module.add_class::<PyMarketOrderBook>()?;
    module.add_class::<PyMarketRollingWindow>()?;
    module.add_class::<PyMarketTicks>()?;
    module.add_class::<PyAlignedWindow>()?;
    module.add_class::<PyRollingWindow>()?;
    module.add_class::<PyMarket>()?;
    module.add_class::<PyRawMarketCandle>()?;
    module.add_class::<PyRawMarketOhlcv>()?;
    module.add_class::<PyRawMarketOrderBook>()?;
    module.add_class::<PyRawMarketQuote>()?;
    module.add_class::<PyRawMarketTick>()?;
    Ok(module)
}
