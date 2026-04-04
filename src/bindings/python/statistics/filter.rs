#[cfg(feature = "python")]
use {
    crate::statistics::filter,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "Filter")]
pub struct PyFilter;

#[cfg(feature = "python")]
#[pymethods]
impl PyFilter {
    pub fn hodrick_prescott_filter(
        &self,
        time_series_values: Vec<f64>,
        lambda: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        filter::hodrick_prescott_filter(&time_series_values, lambda)
    }
    pub fn wiener_filter(&self, time_series_values: Vec<f64>, window_size: usize) -> Vec<f64> {
        filter::wiener_filter(&time_series_values, window_size)
    }
}
