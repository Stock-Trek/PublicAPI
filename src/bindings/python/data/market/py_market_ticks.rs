#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::data::market::py_raw_market_tick::PyRawMarketTick,
        data::market::market_ticks::MarketTicks,
    },
    pyo3::{prelude::*, pyclass, pymethods, types::PyList, Python},
};

#[cfg(feature = "python")]
#[pyclass(name = "MarketTicks")]
pub struct PyMarketTicks {
    inner: MarketTicks,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMarketTicks {
    pub fn exact(&self, py: Python<'_>) -> Py<PyList> {
        let py_ticks: Vec<_> = self
            .inner
            .exact()
            .iter()
            .map(|tick| PyRawMarketTick::from(tick))
            .collect();
        PyList::new(py, py_ticks).unwrap().into()
    }
    pub fn bids(&self, py: Python<'_>) -> Py<PyList> {
        let list = PyList::empty(py);
        for bid in self.inner.bids() {
            let bid_list = PyList::empty(py);
            bid_list.add(bid.0).unwrap();
            bid_list.add(bid.1).unwrap();
            bid_list.add(bid.2).unwrap();
            list.add(bid_list).unwrap();
        }
        list.into()
    }
    pub fn asks(&self, py: Python<'_>) -> Py<PyList> {
        let list = PyList::empty(py);
        for bid in self.inner.asks() {
            let ask_list = PyList::empty(py);
            ask_list.add(bid.0).unwrap();
            ask_list.add(bid.1).unwrap();
            ask_list.add(bid.2).unwrap();
            list.add(ask_list).unwrap();
        }
        list.into()
    }
    pub fn lasts(&self, py: Python<'_>) -> Py<PyList> {
        let list = PyList::empty(py);
        for last in self.inner.lasts() {
            let last_list = PyList::empty(py);
            last_list.add(last.0).unwrap();
            last_list.add(last.1).unwrap();
            last_list.add(last.2).unwrap();
            list.add(last_list).unwrap();
        }
        list.into()
    }
}

#[cfg(feature = "python")]
impl From<&MarketTicks> for PyMarketTicks {
    fn from(ticks: &MarketTicks) -> Self {
        Self {
            inner: ticks.to_owned(),
        }
    }
}

#[cfg(feature = "python")]
impl PyMarketTicks {
    pub fn new_py(py: Python<'_>, ticks: &MarketTicks) -> PyResult<Py<Self>> {
        Py::new(py, PyMarketTicks::from(ticks))
    }
}
