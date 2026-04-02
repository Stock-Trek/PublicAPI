#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::convert::rust_decimal_to_py,
        data::market_data::raw_market_ohlcv::RawMarketOhlcv,
    },
    pyo3::{prelude::*, pyclass, pymethods, PyResult, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "RawMarketOhlcv")]
pub struct PyRawMarketOhlcv {
    inner: RawMarketOhlcv,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRawMarketOhlcv {
    pub fn open(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.open())
    }
    pub fn high(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.high())
    }
    pub fn low(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.low())
    }
    pub fn close(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.close())
    }
    pub fn volume(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.volume())
    }
    pub fn quote_volume(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.quote_volume())
    }
    pub fn vwap(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        rust_decimal_to_py(py, self.inner.vwap())
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<&RawMarketOhlcv> for PyRawMarketOhlcv {
    fn from(ohlcv: &RawMarketOhlcv) -> Self {
        Self {
            inner: ohlcv.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyRawMarketOhlcv {
    pub fn new_py(py: Python<'_>, ohlcv: &RawMarketOhlcv) -> PyResult<Py<Self>> {
        Py::new(py, PyRawMarketOhlcv::from(ohlcv))
    }
}
