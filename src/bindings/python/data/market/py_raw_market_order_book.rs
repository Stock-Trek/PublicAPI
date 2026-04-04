#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::py_raw_market_quote::PyRawMarketQuote,
        dto::raw_market_order_book::RawMarketOrderBook,
    },
    pyo3::{prelude::*, pyclass, pymethods, types::PyList, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "RawMarketOrderBook")]
pub struct PyRawMarketOrderBook {
    inner: RawMarketOrderBook,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRawMarketOrderBook {
    pub fn bids(&self, py: Python<'_>) -> Py<PyList> {
        let py_bids: Vec<_> = self
            .inner
            .bids()
            .iter()
            .map(|bid| PyRawMarketQuote::from(bid))
            .collect();
        PyList::new(py, py_bids).unwrap().into()
    }
    pub fn asks(&self, py: Python<'_>) -> Py<PyList> {
        let py_bids: Vec<_> = self
            .inner
            .asks()
            .iter()
            .map(|ask| PyRawMarketQuote::from(ask))
            .collect();
        PyList::new(py, py_bids).unwrap().into()
    }
}

#[cfg(feature = "python")]
impl From<&RawMarketOrderBook> for PyRawMarketOrderBook {
    fn from(order_book: &RawMarketOrderBook) -> Self {
        Self {
            inner: order_book.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyRawMarketOrderBook {
    pub fn new_py(py: Python<'_>, order_book: &RawMarketOrderBook) -> PyResult<Py<Self>> {
        Py::new(py, PyRawMarketOrderBook::from(order_book))
    }
}
