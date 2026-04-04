#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::statistics::{
            advanced::PyAdvanced, decompose::PyDecompose, evaluation::PyEvaluation,
            exponential_smoothing::PyExponentialSmoothing, filter::PyFilter,
            frequency::PyFrequency, hypothesis::PyHypothesis, moving_average::PyMovingAverage,
            time_series::PyTimeSeries, transformation::PyTransformation, wavelet::PyWavelet,
        },
        statistics::stats,
    },
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "Stats")]
pub struct PyStats {
    pub advanced: PyAdvanced,
    pub decompose: PyDecompose,
    pub evaluation: PyEvaluation,
    pub exponential_smoothing: PyExponentialSmoothing,
    pub filter: PyFilter,
    pub frequency: PyFrequency,
    pub hypothesis: PyHypothesis,
    pub moving_average: PyMovingAverage,
    pub time_series: PyTimeSeries,
    pub transformation: PyTransformation,
    pub wavelet: PyWavelet,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStats {
    pub fn correlation(&self, first: Vec<f64>, second: Vec<f64>) -> PyResult<f64> {
        stats::correlation(&first, &second).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn covariance(&self, first: Vec<f64>, second: Vec<f64>) -> PyResult<f64> {
        stats::covariance(&first, &second).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn kurtosis(&self, values: Vec<f64>) -> PyResult<f64> {
        stats::kurtosis(&values).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn mean(&self, values: Vec<f64>) -> PyResult<f64> {
        stats::mean(&values).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn skewness(&self, values: Vec<f64>) -> PyResult<f64> {
        stats::skewness(&values).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn standard_deviation(
        &self,
        values: Vec<f64>,
        delta_degrees_of_freedom: usize,
    ) -> PyResult<f64> {
        stats::standard_deviation(&values, delta_degrees_of_freedom)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn variance(&self, values: Vec<f64>, delta_degrees_of_freedom: usize) -> PyResult<f64> {
        stats::variance(&values, delta_degrees_of_freedom)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
