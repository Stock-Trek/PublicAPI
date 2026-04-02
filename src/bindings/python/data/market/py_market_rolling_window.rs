#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::{
            convert::vec_to_list, py_market_window::PyRollingWindow,
            py_raw_market_candle::PyRawMarketCandle,
        },
        data::market_data::market_rolling_window::MarketRollingWindow,
    },
    pyo3::{
        prelude::*,
        types::{PyDict, PyList},
    },
};

#[cfg(feature = "python")]
#[pyclass(name = "MarketRollingWindow")]
pub struct PyMarketRollingWindow {
    inner: MarketRollingWindow,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMarketRollingWindow {
    pub fn exact(&self, py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        self.inner.exact().iter().for_each(|(window, candle)| {
            let py_window = PyRollingWindow::from(window);
            let py_candle = PyRawMarketCandle::from(candle);
            dict.set_item(py_window, py_candle).unwrap();
        });
        dict.into()
    }
    pub fn ohlcv(&self, py: Python<'_>, window: &PyRollingWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.ohlcv(window.into()))
    }
}

#[cfg(feature = "python")]
impl From<&MarketRollingWindow> for PyMarketRollingWindow {
    fn from(window: &MarketRollingWindow) -> Self {
        Self {
            inner: window.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyMarketRollingWindow {
    pub fn new_py(py: Python<'_>, window: &MarketRollingWindow) -> PyResult<Py<Self>> {
        Py::new(py, PyMarketRollingWindow::from(window))
    }
}
