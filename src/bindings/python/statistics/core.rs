#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn mean(values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::stats::mean(&values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn variance(values: Vec<f64>, delta_degrees_of_freedom: usize) -> PyResult<f64> {
    let result = crate::statistics::stats::variance(&values, delta_degrees_of_freedom)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn standard_deviation(values: Vec<f64>, delta_degrees_of_freedom: usize) -> PyResult<f64> {
    let result = crate::statistics::stats::standard_deviation(&values, delta_degrees_of_freedom)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn covariance(first: Vec<f64>, second: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::stats::covariance(&first, &second)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn correlation(first: Vec<f64>, second: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::stats::correlation(&first, &second)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn skewness(values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::stats::skewness(&values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn kurtosis(values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::stats::kurtosis(&values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
