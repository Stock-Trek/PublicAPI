#[cfg(feature = "python")]
use {
    crate::statistics::time_series,
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "TimeSeries")]
pub struct PyTimeSeries;

#[cfg(feature = "python")]
#[pymethods]
impl PyTimeSeries {
    pub fn autocorrelation(&self, values: Vec<f64>, lag: usize) -> PyResult<f64> {
        time_series::autocorrelation(&values, lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn autocovariance(&self, values: Vec<f64>, lag: usize) -> PyResult<f64> {
        time_series::autocovariance(&values, lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn cross_correlation(
        &self,
        first: Vec<f64>,
        second: Vec<f64>,
        lag: isize,
    ) -> PyResult<f64> {
        time_series::cross_correlation(&first, &second, lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn partial_autocorrelation(&self, values: Vec<f64>, max_lag: usize) -> PyResult<Vec<f64>> {
        time_series::partial_autocorrelation(&values, max_lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
