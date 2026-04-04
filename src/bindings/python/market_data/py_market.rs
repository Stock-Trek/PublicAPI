#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::{
            dto::convert::rust_decimal_to_py,
            market_data::{
                py_market_aligned_window::PyMarketAlignedWindow,
                py_market_order_book::PyMarketOrderBook,
                py_market_rolling_window::PyMarketRollingWindow, py_market_ticks::PyMarketTicks,
            },
        },
        market_data::market::Market,
    },
    pyo3::{prelude::*, pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "Market")]
pub struct PyMarket {
    inner: Market,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMarket {
    pub fn base_increment(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.base_increment())
    }
    pub fn quote_increment(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.quote_increment())
    }
    pub fn ticks(&self, py: Python<'_>) -> PyResult<Py<PyMarketTicks>> {
        PyMarketTicks::new_py(py, self.inner.ticks())
    }
    pub fn rolling(&self, py: Python<'_>) -> PyResult<Py<PyMarketRollingWindow>> {
        PyMarketRollingWindow::new_py(py, self.inner.rolling())
    }
    pub fn aligned(&self, py: Python<'_>) -> PyResult<Py<PyMarketAlignedWindow>> {
        PyMarketAlignedWindow::new_py(py, self.inner.aligned())
    }
    pub fn order_book(&self, py: Python<'_>) -> PyResult<Py<PyMarketOrderBook>> {
        PyMarketOrderBook::new_py(py, self.inner.order_book())
    }
}

#[cfg(feature = "python")]
impl From<&Market> for PyMarket {
    fn from(market: &Market) -> Self {
        Self {
            inner: market.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyMarket {
    pub fn new_py(py: Python<'_>, market: &Market) -> PyResult<Py<Self>> {
        Py::new(py, PyMarket::from(market))
    }
}
