#[cfg(feature = "python")]
use pyo3::{exceptions::PyRuntimeError, pyfunction, PyResult};

#[cfg(feature = "python")]
#[pyfunction]
pub fn shannon_entropy(probability_distribution: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::advanced::shannon_entropy(&probability_distribution)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn sample_entropy(
    time_series_values: Vec<f64>,
    embedding_dimension: usize,
    tolerance: f64,
) -> PyResult<f64> {
    let result = crate::statistics::advanced::sample_entropy(
        &time_series_values,
        embedding_dimension,
        tolerance,
    )
    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn hurst_exponent(time_series_values: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::advanced::hurst_exponent(&time_series_values)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn mutual_information(first_series: Vec<f64>, second_series: Vec<f64>) -> PyResult<f64> {
    let result = crate::statistics::advanced::mutual_information(&first_series, &second_series)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(result)
}
