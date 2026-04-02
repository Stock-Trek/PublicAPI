#[cfg(feature = "python")]
use pyo3::{prelude::*, pyclass, pymethods};

#[cfg(feature = "python")]
use crate::bindings::python::data::market::convert::rust_decimal_to_py;
use crate::data::market_data::raw_market_quote::RawMarketQuote;

#[cfg(feature = "python")]
#[pyclass(name = "RawMarketQuote")]
pub struct PyRawMarketQuote {
    inner: RawMarketQuote,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRawMarketQuote {
    pub fn price(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.price())
    }
    pub fn quantity(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.quantity())
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<&RawMarketQuote> for PyRawMarketQuote {
    fn from(market_quote: &RawMarketQuote) -> Self {
        Self {
            inner: market_quote.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyRawMarketQuote {
    pub fn new_py(py: Python<'_>, market_quote: &RawMarketQuote) -> PyResult<Py<Self>> {
        Py::new(py, PyRawMarketQuote::from(market_quote))
    }
}
