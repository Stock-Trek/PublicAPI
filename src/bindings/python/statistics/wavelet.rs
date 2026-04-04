#[cfg(feature = "python")]
use {
    crate::statistics::wavelet,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "Wavelet")]
pub struct PyWavelet;

#[cfg(feature = "python")]
#[pymethods]
impl PyWavelet {
    pub fn continuous_wavelet_transform(
        &self,
        time_series_values: Vec<f64>,
        scale_values: Vec<f64>,
        wavelet_name: String,
    ) -> Vec<Vec<f64>> {
        wavelet::continuous_wavelet_transform(&time_series_values, &scale_values, &wavelet_name)
    }
    pub fn discrete_wavelet_transform(
        &self,
        time_series_values: Vec<f64>,
        wavelet_name: String,
    ) -> (Vec<f64>, Vec<f64>) {
        wavelet::discrete_wavelet_transform(&time_series_values, &wavelet_name)
    }
}
