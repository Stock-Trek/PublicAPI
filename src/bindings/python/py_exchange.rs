#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::{market_data::py_market::PyMarket, py_trading_pair::PyTradingPair},
        exchange::{Exchange, ExchangeName},
    },
    pyo3::{prelude::*, types::PyDict},
};

#[cfg(feature = "python")]
#[pyclass(name = "RollingWindow")]
pub struct PyExchangeName {
    inner: ExchangeName,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyExchangeName {
    pub const BINANCE: Self = Self {
        inner: ExchangeName::Binance,
    };
}

#[cfg(feature = "python")]
impl From<&PyExchangeName> for ExchangeName {
    fn from(py: &PyExchangeName) -> Self {
        py.inner
    }
}

#[cfg(feature = "python")]
impl From<&ExchangeName> for PyExchangeName {
    fn from(inner: &ExchangeName) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "StockTrekContext", from_py_object)]
#[derive(Clone)]
pub struct PyExchange {
    inner: Exchange,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyExchange {
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

#[cfg(feature = "python")]
impl From<&Exchange> for PyExchange {
    fn from(exchange: &Exchange) -> Self {
        Self {
            inner: exchange.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyExchange {
    pub fn new_py(py: Python<'_>, exchange: &Exchange) -> PyResult<Py<Self>> {
        Py::new(py, PyExchange::from(exchange))
    }
}
