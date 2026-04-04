#[cfg(feature = "python")]
use {
    crate::statistics::hypothesis,
    pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyResult},
};

#[cfg(feature = "python")]
#[pyclass(name = "Hypothesis")]
pub struct PyHypothesis;

#[cfg(feature = "python")]
#[pymethods]
impl PyHypothesis {
    pub fn augmented_dickey_fuller(
        &self,
        time_series_values: Vec<f64>,
        maximum_lag: usize,
    ) -> PyResult<(f64, f64)> {
        hypothesis::augmented_dickey_fuller(&time_series_values, maximum_lag)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn durbin_watson(&self, residual_values: Vec<f64>) -> PyResult<f64> {
        hypothesis::durbin_watson(&residual_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn jarque_bera(&self, time_series_values: Vec<f64>) -> PyResult<(f64, f64)> {
        hypothesis::jarque_bera(&time_series_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn kwiatkowski_phillips_schmidt_shin(
        &self,
        time_series_values: Vec<f64>,
    ) -> PyResult<(f64, f64)> {
        hypothesis::kwiatkowski_phillips_schmidt_shin(&time_series_values)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
    pub fn ljung_box(&self, time_series_values: Vec<f64>, number_of_lags: usize) -> PyResult<f64> {
        hypothesis::ljung_box(&time_series_values, number_of_lags)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
