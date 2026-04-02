#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::{
            convert::vec_to_list, py_market_window::PyAlignedWindow,
            py_raw_market_candle::PyRawMarketCandle,
        },
        data::market::market_aligned_window::MarketAlignedWindow,
    },
    pyo3::{
        prelude::*,
        pyclass,
        types::{PyDict, PyList},
        Python,
    },
};

#[cfg(feature = "python")]
#[pyclass(name = "MarketAlignedWindow")]
pub struct PyMarketAlignedWindow {
    inner: MarketAlignedWindow,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMarketAlignedWindow {
    pub fn exact(&self, py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        self.inner.exact().iter().for_each(|(window, candles)| {
            let py_window = PyAlignedWindow::from(window);
            let py_candles = PyList::empty(py);
            for candle in candles {
                py_candles.append(PyRawMarketCandle::from(candle)).unwrap();
            }
            dict.set_item(py_window, py_candles).unwrap();
        });
        dict.into()
    }
    pub fn opens(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.opens(window.into()))
    }
    pub fn highs(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.highs(window.into()))
    }
    pub fn lows(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.lows(window.into()))
    }
    pub fn closes(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.closes(window.into()))
    }
    pub fn volumes(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.volumes(window.into()))
    }
    pub fn quote_volumes(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.quote_volumes(window.into()))
    }
    pub fn vwaps(&self, py: Python<'_>, window: &PyAlignedWindow) -> Py<PyList> {
        vec_to_list(py, self.inner.vwaps(window.into()))
    }
}

#[cfg(feature = "python")]
impl From<&MarketAlignedWindow> for PyMarketAlignedWindow {
    fn from(window: &MarketAlignedWindow) -> Self {
        Self {
            inner: window.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyMarketAlignedWindow {
    pub fn new_py(py: Python<'_>, window: &MarketAlignedWindow) -> PyResult<Py<Self>> {
        Py::new(py, PyMarketAlignedWindow::from(window))
    }
}
