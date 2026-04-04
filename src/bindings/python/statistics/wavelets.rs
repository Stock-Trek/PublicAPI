#[cfg(feature = "python")]
use pyo3::pyfunction;

#[cfg(feature = "python")]
#[pyfunction]
pub fn discrete_wavelet_transform(
    time_series_values: Vec<f64>,
    wavelet_name: String,
) -> (Vec<f64>, Vec<f64>) {
    crate::statistics::wavelet::discrete_wavelet_transform(&time_series_values, &wavelet_name)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn continuous_wavelet_transform(
    time_series_values: Vec<f64>,
    scale_values: Vec<f64>,
    wavelet_name: String,
) -> Vec<Vec<f64>> {
    crate::statistics::wavelet::continuous_wavelet_transform(
        &time_series_values,
        &scale_values,
        &wavelet_name,
    )
}
