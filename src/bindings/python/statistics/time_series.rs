#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn autocovariance(values: Vec<f64>, lag: usize) -> PyResult<f64> {
    let result = crate::statistics::time_series::autocovariance(&values, lag)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn autocorrelation(values: Vec<f64>, lag: usize) -> PyResult<f64> {
    let result = crate::statistics::time_series::autocorrelation(&values, lag)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn partial_autocorrelation(values: Vec<f64>, max_lag: usize) -> PyResult<Vec<f64>> {
    let result = crate::statistics::time_series::partial_autocorrelation(&values, max_lag)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn cross_correlation(first: Vec<f64>, second: Vec<f64>, lag: isize) -> PyResult<f64> {
    let result = crate::statistics::time_series::cross_correlation(&first, &second, lag)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
