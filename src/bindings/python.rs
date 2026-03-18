#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
pub fn stock_trek(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::data::add_submodule(py, m)?;

    Ok(())
}
