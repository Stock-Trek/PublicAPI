#[cfg(feature = "python")]
use {
    crate::bindings::python::dto::{
        py_raw_market_candle::PyRawMarketCandle, py_raw_market_ohlcv::PyRawMarketOhlcv,
        py_raw_market_order_book::PyRawMarketOrderBook, py_raw_market_quote::PyRawMarketQuote,
        py_raw_market_tick::PyRawMarketTick,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "dto")?;
    module.add_class::<PyRawMarketCandle>()?;
    module.add_class::<PyRawMarketOhlcv>()?;
    module.add_class::<PyRawMarketOrderBook>()?;
    module.add_class::<PyRawMarketQuote>()?;
    module.add_class::<PyRawMarketTick>()?;
    Ok(module)
}
