#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn augmented_dickey_fuller(
    time_series_values: Vec<f64>,
    maximum_lag: usize,
) -> PyResult<(f64, f64)> {
    let result =
        crate::statistics::hypothesis::augmented_dickey_fuller(&time_series_values, maximum_lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn kwiatkowski_phillips_schmidt_shin(time_series_values: Vec<f64>) -> PyResult<(f64, f64)> {
    let result =
        crate::statistics::hypothesis::kwiatkowski_phillips_schmidt_shin(&time_series_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn ljung_box(time_series_values: Vec<f64>, number_of_lags: usize) -> PyResult<f64> {
    let result = crate::statistics::hypothesis::ljung_box(&time_series_values, number_of_lags)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn durbin_watson(residual_values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::hypothesis::durbin_watson(&residual_values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn jarque_bera(time_series_values: Vec<f64>) -> PyResult<(f64, f64)> {
    let result = crate::statistics::hypothesis::jarque_bera(&time_series_values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
