#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::{
            convert::vec_to_list, py_raw_market_order_book::PyRawMarketOrderBook,
        },
        data::market::market_order_book::MarketOrderBook,
    },
    pyo3::{prelude::*, pyclass, pymethods, types::PyList, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "MarketOrderBook")]
pub struct PyMarketOrderBook {
    inner: MarketOrderBook,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMarketOrderBook {
    pub fn exact(&self, py: Python<'_>) -> PyResult<Py<PyRawMarketOrderBook>> {
        PyRawMarketOrderBook::new_py(py, self.inner.exact())
    }
    pub fn bids(&self, py: Python<'_>) -> Py<PyList> {
        vec_to_list(py, self.inner.bids())
    }
    pub fn asks(&self, py: Python<'_>) -> Py<PyList> {
        vec_to_list(py, self.inner.asks())
    }
}

#[cfg(feature = "python")]
impl From<&MarketOrderBook> for PyMarketOrderBook {
    fn from(order_book: &MarketOrderBook) -> Self {
        Self {
            inner: order_book.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyMarketOrderBook {
    pub fn new_py(py: Python<'_>, order_book: &MarketOrderBook) -> PyResult<Py<Self>> {
        Py::new(py, PyMarketOrderBook::from(order_book))
    }
}
