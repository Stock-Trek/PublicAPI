#[cfg(feature = "python")]
use {
    crate::bindings::python::statistics::{
        advanced::PyAdvanced, decompose::PyDecompose, evaluation::PyEvaluation,
        exponential_smoothing::PyExponentialSmoothing, filter::PyFilter, frequency::PyFrequency,
        hypothesis::PyHypothesis, moving_average::PyMovingAverage, stats::PyStats,
        time_series::PyTimeSeries, transformation::PyTransformation, wavelet::PyWavelet,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "statistics")?;
    module.add_class::<PyAdvanced>()?;
    module.add_class::<PyDecompose>()?;
    module.add_class::<PyEvaluation>()?;
    module.add_class::<PyExponentialSmoothing>()?;
    module.add_class::<PyFilter>()?;
    module.add_class::<PyFrequency>()?;
    module.add_class::<PyHypothesis>()?;
    module.add_class::<PyMovingAverage>()?;
    module.add_class::<PyStats>()?;
    module.add_class::<PyTimeSeries>()?;
    module.add_class::<PyTransformation>()?;
    module.add_class::<PyWavelet>()?;
    Ok(module)
}
