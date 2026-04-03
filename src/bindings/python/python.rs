#[cfg(feature = "python")]
use {
    crate::bindings::python::{py_algorithm::PyStockTrekAlgorithm, py_context::PyStockTrekContext},
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
#[pymodule]
pub fn _stock_trek(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_submodule(&super::data::python::create_module(py)?)?;
    module.add_submodule(&super::schemas::python::create_module(py)?)?;
    module.add_submodule(&super::statistics::python::create_module(py)?)?;
    module.add_class::<PyStockTrekAlgorithm>()?;
    module.add_class::<PyStockTrekContext>()?;
    Ok(())
}
