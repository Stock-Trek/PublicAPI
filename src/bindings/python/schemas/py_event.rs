#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::schemas::py_signal::{PyMetadata, PyStockTrekSignal},
        schemas::signal::StockTrekEvent,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyStockTrekEvent {
    #[pyo3(get, set)]
    pub metadata: PyMetadata,
    #[pyo3(get, set)]
    pub signal: PyStockTrekSignal,
}

#[cfg(feature = "python")]
impl From<StockTrekEvent> for PyStockTrekEvent {
    fn from(e: StockTrekEvent) -> Self {
        Self {
            metadata: e.metadata.into(),
            signal: e.signal.into(),
        }
    }
}
