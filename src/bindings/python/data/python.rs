#[cfg(feature = "python")]
use {crate::bindings::python::data::py_trading_pair::PyTradingPair, pyo3::prelude::*};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "data")?;
    module.add_submodule(&super::market::python::create_module(py)?)?;
    module.add_class::<PyTradingPair>()?;
    Ok(module)
}
