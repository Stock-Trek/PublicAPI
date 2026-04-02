#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::{market::py_market::PyMarket, py_trading_pair::PyTradingPair},
        data::context::StockTrekContext,
    },
    pyo3::{prelude::*, types::PyDict},
};

#[cfg(feature = "python")]
#[pyclass(name = "StockTrekContext")]
pub struct PyStockTrekContext {
    inner: StockTrekContext,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStockTrekContext {
    pub fn markets(&self, py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        for (key, value) in self.inner.markets() {
            let py_key = PyTradingPair::from(key);
            let py_market = PyMarket::from(value);
            dict.set_item(py_key, py_market).unwrap();
        }
        dict.into()
    }
}
