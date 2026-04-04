#[cfg(feature = "python")]
use {
    crate::bindings::python::{py_context::PyStockTrekContext, py_signal::PyStockTrekSignal},
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
#[pyclass]
pub struct PyStockTrekAlgorithm {
    py_callable: Py<PyAny>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStockTrekAlgorithm {
    #[new]
    fn new(py_callable: Py<PyAny>) -> Self {
        Self { py_callable }
    }

    fn create_signal(
        &self,
        py: Python<'_>,
        context: PyStockTrekContext,
    ) -> PyResult<PyStockTrekSignal> {
        let py_result = self
            .py_callable
            .call_method(py, "create_signal", (context,), None)?;
        let bound_result = py_result.into_bound(py);
        Ok(bound_result.extract::<PyStockTrekSignal>()?)
    }
}
