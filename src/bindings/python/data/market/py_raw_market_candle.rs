#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::py_raw_market_ohlcv::PyRawMarketOhlcv,
        dto::raw_market_candle::RawMarketCandle, prelude::TimestampMillis,
    },
    pyo3::{prelude::*, pyclass, pymethods, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "RawMarketCandle")]
pub struct PyRawMarketCandle {
    inner: RawMarketCandle,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRawMarketCandle {
    pub fn start_time_millis_inc(&self, _py: Python<'_>) -> TimestampMillis {
        self.inner.start_time_millis_inc()
    }
    pub fn end_time_millis_exc(&self, _py: Python<'_>) -> TimestampMillis {
        self.inner.end_time_millis_exc()
    }
    pub fn duration_millis(&self, _py: Python<'_>) -> TimestampMillis {
        self.inner.duration_millis()
    }
    pub fn is_candle_closed(&self, _py: Python<'_>) -> bool {
        self.inner.is_candle_closed()
    }
    pub fn ohlcv(&self, py: Python<'_>) -> PyResult<Py<PyRawMarketOhlcv>> {
        PyRawMarketOhlcv::new_py(py, self.inner.ohlcv())
    }
    pub fn trade_count(&self, _py: Python<'_>) -> u64 {
        self.inner.trade_count()
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<&RawMarketCandle> for PyRawMarketCandle {
    fn from(market_candle: &RawMarketCandle) -> Self {
        Self {
            inner: market_candle.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyRawMarketCandle {
    pub fn new_py(py: Python<'_>, market_candle: &RawMarketCandle) -> PyResult<Py<Self>> {
        Py::new(py, PyRawMarketCandle::from(market_candle))
    }
}
