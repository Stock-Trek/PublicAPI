#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn lag(values: Vec<f64>, lag: usize) -> PyResult<Vec<Option<f64>>> {
    let result = crate::statistics::transformations::lag(&values, lag)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn difference(values: Vec<f64>, order: usize) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::difference(&values, order)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn seasonal_difference(values: Vec<f64>, period: usize) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::seasonal_difference(&values, period)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn logarithm(values: Vec<f64>) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::logarithm(&values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn box_cox(values: Vec<f64>, lambda: f64) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::box_cox(&values, lambda)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn detrend_linear(values: Vec<f64>) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::detrend_linear(&values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn rolling_mean(values: Vec<f64>, window: usize) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::rolling_mean(&values, window)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn rolling_standard_deviation(values: Vec<f64>, window: usize) -> PyResult<Vec<f64>> {
    let result = crate::statistics::transformations::rolling_standard_deviation(&values, window)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
