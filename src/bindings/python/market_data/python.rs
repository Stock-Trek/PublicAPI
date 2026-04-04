#[cfg(feature = "python")]
use {
    crate::bindings::python::market_data::{
        py_market::PyMarket,
        py_market_aligned_window::PyMarketAlignedWindow,
        py_market_order_book::PyMarketOrderBook,
        py_market_rolling_window::PyMarketRollingWindow,
        py_market_ticks::PyMarketTicks,
        py_market_window::{PyAlignedWindow, PyRollingWindow},
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "market_data")?;
    module.add_class::<PyMarketAlignedWindow>()?;
    module.add_class::<PyMarketOrderBook>()?;
    module.add_class::<PyMarketRollingWindow>()?;
    module.add_class::<PyMarketTicks>()?;
    module.add_class::<PyAlignedWindow>()?;
    module.add_class::<PyRollingWindow>()?;
    module.add_class::<PyMarket>()?;
    Ok(module)
}
