#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
pub fn _stock_trek(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_submodule(&super::data::python::create_module(py)?)?;
    module.add_submodule(&super::statistics::python::create_module(py)?)?;
    Ok(())
}
