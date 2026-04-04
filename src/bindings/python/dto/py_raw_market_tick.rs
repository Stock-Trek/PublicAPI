#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::dto::py_raw_market_quote::PyRawMarketQuote,
        dto::raw_market_tick::RawMarketTick, prelude::TimestampMillis,
    },
    pyo3::{prelude::*, pyclass, pymethods, PyResult, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "RawMarketTick")]
pub struct PyRawMarketTick {
    inner: RawMarketTick,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRawMarketTick {
    pub fn timestamp_millis(&self, _py: Python<'_>) -> TimestampMillis {
        self.inner.timestamp_millis()
    }
    pub fn bid(&self, py: Python<'_>) -> PyResult<Py<PyRawMarketQuote>> {
        PyRawMarketQuote::new_py(py, self.inner.bid())
    }
    pub fn ask(&self, py: Python<'_>) -> PyResult<Py<PyRawMarketQuote>> {
        PyRawMarketQuote::new_py(py, self.inner.ask())
    }
    pub fn last(&self, py: Python<'_>) -> PyResult<Py<PyRawMarketQuote>> {
        PyRawMarketQuote::new_py(py, self.inner.last())
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<&RawMarketTick> for PyRawMarketTick {
    fn from(market_tick: &RawMarketTick) -> Self {
        Self {
            inner: market_tick.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyRawMarketTick {
    pub fn new_py(py: Python<'_>, market_tick: &RawMarketTick) -> PyResult<Py<Self>> {
        Py::new(py, PyRawMarketTick::from(market_tick))
    }
}
