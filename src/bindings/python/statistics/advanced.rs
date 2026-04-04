#[cfg(feature = "python")]
use {
    crate::statistics::advanced,
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "Advanced")]
pub struct PyAdvanced;

#[cfg(feature = "python")]
#[pymethods]
impl PyAdvanced {
    pub fn shannon_entropy(&self, probability_distribution: Vec<f64>) -> PyResult<f64> {
        let result = advanced::shannon_entropy(&probability_distribution)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn sample_entropy(
        &self,
        time_series_values: Vec<f64>,
        embedding_dimension: usize,
        tolerance: f64,
    ) -> PyResult<f64> {
        let result = advanced::sample_entropy(&time_series_values, embedding_dimension, tolerance)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn hurst_exponent(&self, time_series_values: Vec<f64>) -> PyResult<f64> {
        let result = advanced::hurst_exponent(&time_series_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
    pub fn mutual_information(
        &self,
        first_series: Vec<f64>,
        second_series: Vec<f64>,
    ) -> PyResult<f64> {
        let result = advanced::mutual_information(&first_series, &second_series)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(result)
    }
}
