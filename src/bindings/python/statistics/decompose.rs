#[cfg(feature = "python")]
use {
    crate::statistics::decompose,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "Decompose")]
pub struct PyDecompose;

#[cfg(feature = "python")]
#[pymethods]
impl PyDecompose {
    pub fn seasonal_decompose(
        &self,
        time_series_values: Vec<f64>,
        seasonal_period_length: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        decompose::seasonal_decompose(&time_series_values, seasonal_period_length)
    }
    pub fn loess_smooth(&self, values: Vec<f64>, span: usize) -> Vec<f64> {
        decompose::loess_smooth(&values, span)
    }
    pub fn seasonal_trend_decomposition_using_loess(
        &self,
        time_series_values: Vec<f64>,
        seasonal_period_length: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        decompose::seasonal_trend_decomposition_using_loess(
            &time_series_values,
            seasonal_period_length,
        )
    }
}
